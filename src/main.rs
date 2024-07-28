mod error;
use error::{Error, Result};

#[macro_use]
extern crate serde_derive;

use serde_bencode::de;
use serde_bytes::ByteBuf;
use sha1::{Digest, Sha1};
use std::io::prelude::*;
use std::net::TcpStream;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Node(String, i64);

#[derive(Debug, Deserialize)]
struct File {
    path: Vec<String>,
    length: i64,
    #[serde(default)]
    md5sum: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Info {
    pub name: String,
    pub pieces: ByteBuf,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    #[serde(default)]
    pub md5sum: Option<String>,
    #[serde(default)]
    pub length: Option<i64>,
    #[serde(default)]
    pub files: Option<Vec<File>>,
    #[serde(default)]
    pub private: Option<u8>,
    #[serde(default)]
    pub path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "root hash")]
    pub root_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Torrent {
    info: Info,
    #[serde(default)]
    announce: Option<String>,
    #[serde(default)]
    nodes: Option<Vec<Node>>,
    #[serde(default)]
    encoding: Option<String>,
    #[serde(default)]
    httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    creation_date: Option<i64>,
    #[serde(rename = "comment")]
    comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    created_by: Option<String>,
}

fn render_torrent(torrent: &Torrent) {
    println!("name:\t\t{}", torrent.info.name);
    println!("announce:\t{:?}", torrent.announce);
    println!("nodes:\t\t{:?}", torrent.nodes);
    if let Some(al) = &torrent.announce_list {
        for a in al {
            println!("announce list:\t{}", a[0]);
        }
    }
    println!("httpseeds:\t{:?}", torrent.httpseeds);
    println!("creation date:\t{:?}", torrent.creation_date);
    println!("comment:\t{:?}", torrent.comment);
    println!("created by:\t{:?}", torrent.created_by);
    println!("encoding:\t{:?}", torrent.encoding);
    println!("piece length:\t{:?}", torrent.info.piece_length);
    println!("private:\t{:?}", torrent.info.private);
    println!("root hash:\t{:?}", torrent.info.root_hash);
    println!("md5sum:\t\t{:?}", torrent.info.md5sum);
    println!("path:\t\t{:?}", torrent.info.path);
    if let Some(files) = &torrent.info.files {
        for f in files {
            println!("file path:\t{:?}", f.path);
            println!("file length:\t{}", f.length);
            println!("file md5sum:\t{:?}", f.md5sum);
        }
    }
}

// The instructor written code starts here
// The above code is picked from an example in serde-bencode crate

// goal: learning to generate a hash in code
fn calculate_info_hash(buffer: Vec<u8>) -> Result<Vec<u8>> {
    // find the info then its hash
    let mut idx: usize = 3;
    while idx < buffer.len() {
        if buffer[idx - 3] == b'i'
            && buffer[idx - 2] == b'n'
            && buffer[idx - 1] == b'f'
            && buffer[idx] == b'o'
        {
            break;
        }
        idx += 1;
    }
    let buffer = buffer[idx + 1..buffer.len() - 1].to_vec();
    let mut hasher = Sha1::new();
    hasher.update(buffer);
    let info_hash = hasher.finalize().to_vec();
    Ok(info_hash)
}

// goal: learning to send a http request in code
fn discover_peers(torrent: &Torrent, info_hash: &Vec<u8>) -> Result<Vec<String>> {
    let tracker_url = torrent.announce.clone().ok_or(Error::MissingTracker)?;
    let tracker_url = format!(
        // TODO still need to automate putting in the total length left
        "{tracker_url}?info_hash={}&peer_id=11111111111111111111&port=6881&uploaded=0&downloaded=0&left=92063&compact=1",
        urlencoding::encode_binary(info_hash)
    );
    let resp = reqwest::blocking::get(tracker_url)?;
    let resp = resp.bytes()?;
    let peers_discovery = de::from_bytes::<PeerDiscoveryResponse>(resp.as_ref())?;
    println!("Peers:");
    let mut peers: Vec<String> = vec![];
    for i in (0..peers_discovery.peers.len()).step_by(6) {
        // to do parse into an ip and port
        peers.push(format!(
            "{}.{}.{}.{}:{}",
            peers_discovery.peers[i],
            peers_discovery.peers[i + 1],
            peers_discovery.peers[i + 2],
            peers_discovery.peers[i + 3],
            (u32::from(peers_discovery.peers[i + 4]) << 8 | peers_discovery.peers[i + 5] as u32),
        ));
    }
    Ok(peers)
}

fn handshake(info_hash: &Vec<u8>, stream: &mut TcpStream) -> Result<()> {
    let mut msg = vec![b'\x13'];
    msg.extend("BitTorrent protocol".as_bytes().to_vec());
    msg.extend(vec![0; 8]);
    msg.extend(info_hash);
    msg.extend(vec![b'1'; 20]);
    stream.write_all(&msg)?;
    let mut buffer = [0; 68];
    stream.read_exact(&mut buffer)?; // buffer now has handshake response
    Ok(())
}

fn main() -> Result<()> {
    // parse the torrent file
    let stdin = std::io::stdin();
    let mut buffer = Vec::new();
    let mut handle = stdin.lock();
    handle.read_to_end(&mut buffer)?;
    let torrent = de::from_bytes::<Torrent>(&buffer)?;
    render_torrent(&torrent);

    // find the info then its hash
    let info_hash = calculate_info_hash(buffer)?;
    println!("Info Hash:\n\t{}", hex::encode(&info_hash));

    // print the piece hashes
    println!("Pieces:");
    for i in (0..torrent.info.pieces.len()).step_by(20) {
        println!("\t{}", hex::encode(&torrent.info.pieces[i..i + 20]));
    }

    // discover peers
    let peers = discover_peers(&torrent, &info_hash)?;
    println!("\t{peers:?}");

    // Client HandShake over a TCP connection
    let peer = peers.last().unwrap();
    let mut stream = TcpStream::connect(peer)?;
    handshake(&info_hash, &mut stream)?;

    // wait for a bitfield message
    let mut buffer = [0; 4];
    stream.read_exact(&mut buffer)?;
    let msg_len = u32::from_be_bytes(buffer) as usize;
    let mut buffer = vec![0; msg_len];
    stream.read_exact(&mut buffer)?;
    println!(
        "bitfield:\n\t{buffer:?} -> size: {msg_len}\n\t| id: {:x} | pieces: {:b}",
        buffer[0], buffer[1]
    );

    // send an interested message here
    let msg = [0, 0, 0, 1, 2];
    stream.write_all(&msg)?;

    // wait for an unchoke message here
    let mut unchoke_msg = [0; 5];
    stream.read_exact(&mut unchoke_msg)?;
    assert_eq!(unchoke_msg, [0, 0, 0, 1, 1]);
    println!("unchoke received:\n\t{unchoke_msg:?}");

    // download a piece
    // request piece 0
    // | msg-length | id: 6 | piece-index | offset | length-requested (2**14 or remaining)
    let mut piece = vec![];
    while piece.len() != torrent.info.piece_length as usize {
        let piece_index = 0u32.to_be_bytes();
        let offset = (piece.len() as u32).to_be_bytes();
        let length_requested =
            (std::cmp::min(1 << 14, torrent.info.piece_length as usize - piece.len()) as u32)
                .to_be_bytes();
        let mut msg = vec![6];
        msg.extend([piece_index, offset, length_requested].concat());
        let msg = [(msg.len() as u32).to_be_bytes().as_slice(), msg.as_slice()].concat();
        stream.write_all(&msg)?;

        // receive the piece
        let mut buf = [0; 4];
        stream.read_exact(&mut buf)?;
        let mut buf = vec![0; u32::from_be_bytes(buf) as usize];
        stream.read_exact(&mut buf)?;
        piece.extend(buf[9..].to_vec());
    }

    // calculate the hash of the piece to verify
    let mut hasher = Sha1::new();
    hasher.update(piece);
    let info_hash = hasher.finalize().to_vec();
    println!("downloaded piece:\n\t0 with {}", hex::encode(info_hash));

    Ok(())
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct PeerDiscoveryResponse {
    #[serde(with = "serde_bytes")]
    pub peers: Vec<u8>,
    #[serde(rename = "min interval")]
    pub min_interval: Option<i64>,
    #[serde(default)]
    pub interval: Option<i64>,
    #[serde(default)]
    pub complete: Option<i64>,
    #[serde(default)]
    pub incomplete: Option<i64>,
}
