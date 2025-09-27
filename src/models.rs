use serde::{Deserialize, Serialize};
use std::collections::*;

#[derive(Serialize, Deserialize, Clone, Eq, Ord, Debug)]
pub struct User {
	pub id: i64,
	pub name: String,
	pub avatar: String,
	pub display_name: String,
}

impl PartialEq for User {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl PartialOrd for User {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.id.partial_cmp(&other.id)
	}
}

#[repr(i32)]
#[derive(PartialEq, Serialize, Deserialize, Clone, Debug)]
pub enum PostType {
	Plugin = 0,
	Module = 1,
	Song = 2,
	Cover = 3,
	Ui = 4,
	Other = 5,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostUpdate {
	pub major: i32,
	pub minor: i32,
	pub changelog: String,
	#[serde(with = "time::serde::rfc3339")]
	pub time: time::OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
	pub id: i32,
	pub name: String,
	pub text: String,
	pub images: Vec<String>,
	pub files: Vec<String>,
	#[serde(with = "time::serde::rfc3339")]
	pub time: time::OffsetDateTime,
	pub post_type: PostType,
	pub download_count: i64,
	pub like_count: i64,
	pub authors: Vec<User>,
	pub dependencies: Option<Vec<Post>>,
	pub dependency_descriptions: Option<HashMap<i32, String>>,
	#[serde(rename = "file_names")]
	pub local_files: Vec<String>,
	pub private: bool,
	pub explicit: bool,
	pub explicit_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
#[repr(i32)]
pub enum ReservationType {
	Song = 0,
	Module = 1,
	CstmItem = 2,
	CosMiku = 10,
	CosRin = 11,
	CosLen = 12,
	CosLuka = 13,
	CosNeru = 14,
	CosHaku = 15,
	CosKaito = 16,
	CosMeiko = 17,
	CosSakine = 18,
	CosTeto = 19,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reservation {
	pub id: i32,
	pub user: i64,
	pub reservation_type: ReservationType,
	#[serde(with = "time::serde::rfc3339")]
	pub time: time::OffsetDateTime,
	pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pv {
	pub uid: String,
	pub post: Option<i32>,
	pub id: i32,
	pub name: String,
	pub name_en: String,
	pub song_info: Option<pv_db::SongInfo>,
	pub song_info_en: Option<pv_db::SongInfo>,
	pub levels: [Option<pv_db::Level>; 5],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllPvs {
	pub reserved_pvs: BTreeMap<i32, Reservation>,
	pub uploaded_pvs: BTreeMap<i32, Vec<Pv>>,
	pub users: BTreeMap<i64, User>,
	pub posts: BTreeMap<i32, Post>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Module {
	pub uid: String,
	pub post: Option<i32>,
	pub id: i32,
	pub module: module_db::Module,
}

#[derive(Serialize, Deserialize)]
pub struct AllModules {
	pub reserved_modules: BTreeMap<i32, Reservation>,
	pub reserved_costumes: BTreeMap<module_db::Chara, BTreeMap<i32, Reservation>>,
	pub uploaded_modules: BTreeMap<i32, Vec<Module>>,
	pub users: BTreeMap<i64, User>,
	pub posts: BTreeMap<i32, Post>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CstmItem {
	pub uid: String,
	pub post: Option<i32>,
	pub id: i32,
	pub cstm_item: module_db::CustomizeItem,
}

#[derive(Serialize, Deserialize)]
pub struct AllCstmItems {
	pub reserved_cstm_items: BTreeMap<i32, Reservation>,
	pub uploaded_cstm_items: BTreeMap<i32, Vec<CstmItem>>,
	pub users: BTreeMap<i64, User>,
	pub posts: BTreeMap<i32, Post>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeilisearchDbEntry {
	pub uid: u64,
	pub post_id: i32,
	pub id: u32,
	pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllDbEntries {
	pub entries: BTreeMap<u32, Vec<MeilisearchDbEntry>>,
	pub posts: BTreeMap<i32, Post>,
}
