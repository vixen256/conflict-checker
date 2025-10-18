mod models;

use eframe::egui;
use egui_extras::{Column, TableBuilder};
use models::*;
use serde::{Deserialize, Serialize};
use std::collections::*;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Config {
	include: Option<Vec<String>>,
}

pub const ROM_DIRS: [&'static str; 31] = [
	".",
	"rom_ps4",
	"rom_ps4_dlc",
	"rom_ps4_fix",
	"rom_ps4_patch",
	"rom_steam",
	"rom_steam_cn",
	"rom_steam_dlc",
	"rom_steam_en",
	"rom_steam_fr",
	"rom_steam_ge",
	"rom_steam_it",
	"rom_steam_kr",
	"rom_steam_region",
	"rom_steam_region_cn",
	"rom_steam_region_cn",
	"rom_steam_region_dlc",
	"rom_steam_region_dlc_kr",
	"rom_steam_region_en",
	"rom_steam_region_fr",
	"rom_steam_region_ge",
	"rom_steam_region_kr",
	"rom_steam_region_sp",
	"rom_steam_region_tw",
	"rom_steam_sp",
	"rom_steam_tw",
	"rom_switch",
	"rom_switch_cn",
	"rom_switch_en",
	"rom_switch_kr",
	"rom_switch_tw",
];

pub const DB_PREFIXES: [&'static str; 21] = [
	"mod_",
	"",
	"end_",
	"mdata_",
	"patch2_",
	"patch_",
	"dlc13_",
	"dlc12_",
	"dlc14_",
	"dlc9_",
	"dlc8_",
	"dlc11_",
	"dlc10_",
	"dlc4_",
	"dlc3B_",
	"dlc7_",
	"privilege_",
	"dlc2A_",
	"dlc1_",
	"dlc3A_",
	"dlc2B_",
];

fn extract_mod_data<P: AsRef<Path>>(dir: P) -> Option<IdData> {
	let mut pvs: BTreeMap<u32, pv_db::Entry> = BTreeMap::new();
	let mut modules: BTreeMap<i32, module_db::Module> = BTreeMap::new();
	let mut cstm_items: BTreeMap<i32, module_db::CustomizeItem> = BTreeMap::new();
	let mut spr_sets: BTreeMap<u32, String> = BTreeMap::new();
	let mut sprites: BTreeMap<u32, String> = BTreeMap::new();
	let mut aet_sets: BTreeMap<u32, String> = BTreeMap::new();
	let mut aet_scenes: BTreeMap<u32, String> = BTreeMap::new();
	let mut objsets: BTreeMap<u32, String> = BTreeMap::new();
	let mut textures: BTreeMap<u32, String> = BTreeMap::new();

	for file in walkdir::WalkDir::new(dir).into_iter().filter(|file| {
		if let Ok(file) = &file {
			file.path().ends_with("config.toml")
		} else {
			false
		}
	}) {
		let file = file.ok()?;
		let file = file.path();
		let data = std::fs::read_to_string(file).ok()?;
		let config: Config = toml::from_str(&data).ok()?;
		let Some(include) = config.include else {
			continue;
		};

		for include in &include {
			for rom in &ROM_DIRS {
				let folder = format!("{}/{include}/{rom}/rom", file.parent()?.to_str()?);
				let path = Path::new(&folder);
				if !path.exists() {
					continue;
				}

				for prefix in &DB_PREFIXES {
					let pv_db = format!("{folder}/{prefix}pv_db.txt");
					let path = Path::new(&pv_db);
					if path.exists() {
						if let Ok(data) = std::fs::read_to_string(&path) {
							let mut pv_db = pv_db::PvDb::from_str(&data)?;
							pvs.append(&mut pv_db.pvs);
						}
					}

					let spr_db = format!("/{folder}/2d/{prefix}spr_db.bin");
					let path = Path::new(&spr_db);
					if path.exists() {
						let spr_db = diva_db::SprDb::from_file(path).ok()?;

						for (id, set) in spr_db.sets {
							spr_sets.insert(id, set.name);

							for (id, sprite) in set.sprites {
								sprites.insert(id, sprite.name);
							}

							for (id, texture) in set.textures {
								sprites.insert(id, texture.name);
							}
						}
					}

					let aet_db = format!("/{folder}/2d/{prefix}aet_db.bin");
					let path = Path::new(&aet_db);
					if path.exists() {
						let aet_db = diva_db::AetDb::from_file(path).ok()?;

						for (id, set) in aet_db.sets {
							aet_sets.insert(id, set.name);

							for (id, scene) in set.scenes {
								aet_scenes.insert(id, scene.name);
							}
						}
					}

					let obj_db = format!("/{folder}/objset/{prefix}obj_db.bin");
					let path = Path::new(&obj_db);
					if path.exists() {
						let obj_db = diva_db::ObjDb::from_file(path).ok()?;

						for (id, set) in obj_db.sets {
							objsets.insert(id, set.name);
						}
					}

					let tex_db = format!("/{folder}/objset/{prefix}tex_db.bin");
					let path = Path::new(&tex_db);
					if path.exists() {
						let tex_db = diva_db::TexDb::from_file(path).ok()?;

						for (id, name) in tex_db.textures {
							textures.insert(id, name);
						}
					}

					let module_tbl = format!("{folder}/{prefix}gm_module_tbl.farc");
					let module_tbl = Path::new(&module_tbl);
					let customize_item_tbl = format!("{folder}/{prefix}gm_customize_item_tbl.farc");
					let customize_item_tbl = Path::new(&customize_item_tbl);
					if module_tbl.exists() || customize_item_tbl.exists() {
						let chritm_prop = format!("{folder}/{prefix}chritm_prop.farc");
						let chritm_prop = Path::new(&chritm_prop);
						let str_array = format!("{folder}/lang2/mod_str_array.toml");
						let str_array = Path::new(&str_array);

						let module_tbl = if module_tbl.exists() {
							Some(module_tbl)
						} else {
							None
						};
						let customize_item_tbl = if customize_item_tbl.exists() {
							Some(customize_item_tbl)
						} else {
							None
						};
						let chritm_prop = if chritm_prop.exists() {
							Some(chritm_prop)
						} else {
							None
						};
						let str_array = if str_array.exists() {
							Some(str_array)
						} else {
							None
						};

						let mut module_db = module_db::ModuleDb::from_files(
							module_tbl,
							customize_item_tbl,
							chritm_prop,
							str_array,
						)?;

						modules.append(&mut module_db.modules);
						cstm_items.append(&mut module_db.cstm_items);
					}
				}
			}
		}
	}

	Some(IdData {
		pvs,
		modules,
		cstm_items,
		spr_sets,
		sprites,
		aet_sets,
		aet_scenes,
		objsets,
		textures,
	})
}

struct DmaData {
	pvs: AllPvs,
	modules: AllModules,
	cstm_items: AllCstmItems,
	spr_sets: AllDbEntries,
	sprites: AllDbEntries,
	aet_sets: AllDbEntries,
	aet_scenes: AllDbEntries,
	objsets: AllDbEntries,
	textures: AllDbEntries,
}

struct IdData {
	pvs: BTreeMap<u32, pv_db::Entry>,
	modules: BTreeMap<i32, module_db::Module>,
	cstm_items: BTreeMap<i32, module_db::CustomizeItem>,
	spr_sets: BTreeMap<u32, String>,
	sprites: BTreeMap<u32, String>,
	aet_sets: BTreeMap<u32, String>,
	aet_scenes: BTreeMap<u32, String>,
	objsets: BTreeMap<u32, String>,
	textures: BTreeMap<u32, String>,
}

struct AppData {
	dma_data: DmaData,
	user_data: Option<IdData>,

	pv_conflicts: bool,
	pv_reservation_conflicts: bool,
	module_conflicts: bool,
	module_reservation_conflicts: bool,
	module_cos_reservation_conflicts: bool,
	cstm_item_conflicts: bool,
	cstm_item_reservation_conflicts: bool,
	spr_set_conflicts: bool,
	sprite_conflicts: bool,
	aet_set_conflicts: bool,
	aet_scene_conflicts: bool,
	objset_conflicts: bool,
	texture_conflicts: bool,
}

fn db_entry_table(
	ui: &mut egui::Ui,
	name: &str,
	mod_data: &BTreeMap<u32, String>,
	dma_data: &AllDbEntries,
) {
	ui.collapsing(format!("{name} Conflicts"), |ui| {
		TableBuilder::new(ui)
			.id_salt(name)
			.striped(true)
			.vscroll(true)
			.column(Column::auto().resizable(true))
			.column(Column::auto().resizable(true))
			.column(Column::auto().resizable(true))
			.column(Column::auto().resizable(true))
			.header(20.0, |mut header| {
				header.col(|ui| {
					ui.heading("ID");
				});
				header.col(|ui| {
					ui.heading("Name");
				});
				header.col(|ui| {
					ui.heading("Conflict Name");
				});
				header.col(|ui| {
					ui.heading("Conflict Source");
				});
			})
			.body(|mut body| {
				for (id, name) in mod_data {
					if let Some(dma_entries) = dma_data.entries.get(id) {
						for dma_entry in dma_entries {
							body.row(20.0, |mut row| {
								row.col(|ui| {
									ui.label(format!("{id}"));
								});
								row.col(|ui| {
									ui.label(name);
								});
								row.col(|ui| {
									ui.label(&dma_entry.name);
								});
								row.col(|ui| {
									if let Some(post) = dma_data.posts.get(&dma_entry.post_id) {
										ui.hyperlink_to(
											&post.name,
											format!("https://divamodarchive.com/post/{}", post.id),
										);
									} else {
										ui.label("MM+");
									}
								});
							});
						}
					}
				}
			});
	});
}

impl eframe::App for AppData {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			if ui.button("Select mod folder").clicked() {
				if let Some(path) = rfd::FileDialog::new().pick_folder() {
					self.user_data = extract_mod_data(path);

					if let Some(user_data) = &self.user_data {
						self.pv_conflicts = user_data.pvs.iter().any(|(id, _)| {
							self.dma_data.pvs.uploaded_pvs.contains_key(&(*id as i32))
						});
						self.pv_reservation_conflicts = user_data.pvs.iter().any(|(id, _)| {
							self.dma_data.pvs.reserved_pvs.contains_key(&(*id as i32))
						});

						// TODO: Check for COS and item conflicts
						self.module_conflicts = user_data
							.modules
							.iter()
							.any(|(id, _)| self.dma_data.modules.uploaded_modules.contains_key(id));
						self.module_reservation_conflicts = user_data
							.modules
							.iter()
							.any(|(id, _)| self.dma_data.modules.reserved_modules.contains_key(id));
						self.module_cos_reservation_conflicts =
							user_data.modules.iter().any(|(_, module)| {
								self.dma_data
									.modules
									.reserved_costumes
									.get(&module.chara)
									.map_or(false, |chara| chara.contains_key(&module.cos.id))
							});

						self.cstm_item_conflicts = user_data.cstm_items.iter().any(|(id, _)| {
							self.dma_data
								.cstm_items
								.uploaded_cstm_items
								.contains_key(id)
						});
						self.cstm_item_reservation_conflicts =
							user_data.cstm_items.iter().any(|(id, _)| {
								self.dma_data
									.cstm_items
									.reserved_cstm_items
									.contains_key(id)
							});

						self.spr_set_conflicts = user_data
							.spr_sets
							.iter()
							.any(|(id, _)| self.dma_data.spr_sets.entries.contains_key(id));
						self.sprite_conflicts = user_data
							.spr_sets
							.iter()
							.any(|(id, _)| self.dma_data.sprites.entries.contains_key(id));
						self.aet_set_conflicts = user_data
							.aet_sets
							.iter()
							.any(|(id, _)| self.dma_data.aet_sets.entries.contains_key(id));
						self.aet_scene_conflicts = user_data
							.aet_scenes
							.iter()
							.any(|(id, _)| self.dma_data.aet_scenes.entries.contains_key(id));
						self.objset_conflicts = user_data
							.objsets
							.iter()
							.any(|(id, _)| self.dma_data.objsets.entries.contains_key(id));
						self.texture_conflicts = user_data
							.textures
							.iter()
							.any(|(id, _)| self.dma_data.textures.entries.contains_key(id));
					}
				}
			}

			if let Some(user_data) = &self.user_data {
				if !self.pv_conflicts
					&& !self.pv_reservation_conflicts
					&& !self.module_conflicts
					&& !self.module_reservation_conflicts
					&& !self.module_cos_reservation_conflicts
					&& !self.cstm_item_conflicts
					&& !self.cstm_item_reservation_conflicts
					&& !self.spr_set_conflicts
					&& !self.sprite_conflicts
					&& !self.aet_set_conflicts
					&& !self.aet_scene_conflicts
					&& !self.objset_conflicts
					&& !self.texture_conflicts
				{
					ui.label("No conflicts found");
				}

				if self.pv_conflicts {
					ui.collapsing("PV Conflicts", |ui| {
						TableBuilder::new(ui)
							.id_salt("PVs")
							.striped(true)
							.vscroll(true)
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.header(20.0, |mut header| {
								header.col(|ui| {
									ui.heading("ID");
								});
								header.col(|ui| {
									ui.heading("Name");
								});
								header.col(|ui| {
									ui.heading("Conflict Name");
								});
								header.col(|ui| {
									ui.heading("Conflict Source");
								});
							})
							.body(|mut body| {
								for (id, pv) in &user_data.pvs {
									if let Some(dma_pvs) =
										self.dma_data.pvs.uploaded_pvs.get(&(*id as i32))
									{
										for dma_pv in dma_pvs {
											body.row(20.0, |mut row| {
												row.col(|ui| {
													ui.label(format!("{id}"));
												});
												row.col(|ui| {
													ui.label(&pv.song_name_en);
												});
												row.col(|ui| {
													ui.label(&dma_pv.name_en);
												});
												row.col(|ui| {
													if let Some(post) = &dma_pv.post {
														if let Some(post) =
															self.dma_data.pvs.posts.get(post)
														{
															ui.hyperlink_to(
																&post.name,
																format!(
																	"https://divamodarchive.com/post/{}",
																	post.id
																),
															);
														} else {
															ui.label("Unknown");
														}
													} else {
														ui.label("MM+");
													}
												});
											});
										}
									}
								}
							});
					});
				}

				if self.pv_reservation_conflicts {
					ui.collapsing("PV Reservation Conflicts", |ui| {
						TableBuilder::new(ui)
							.id_salt("PVReservations")
							.striped(true)
							.vscroll(true)
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.header(20.0, |mut header| {
								header.col(|ui| {
									ui.heading("ID");
								});
								header.col(|ui| {
									ui.heading("Name");
								});
								header.col(|ui| {
									ui.heading("Conflict Label");
								});
								header.col(|ui| {
									ui.heading("Conflict User");
								});
							})
							.body(|mut body| {
								for (id, pv) in &user_data.pvs {
									if let Some(reservation) =
										self.dma_data.pvs.reserved_pvs.get(&(*id as i32))
									{
										body.row(20.0, |mut row| {
											row.col(|ui| {
												ui.label(format!("{id}"));
											});
											row.col(|ui| {
												ui.label(&pv.song_name_en);
											});
											row.col(|ui| {
												if let Some(label) = &reservation.label {
													ui.label(label);
												}
											});
											row.col(|ui| {
												if let Some(user) =
													self.dma_data.pvs.users.get(&reservation.user)
												{
													ui.label(&user.display_name);
												} else {
													ui.label("Unknown");
												}
											});
										});
									}
								}
							});
					});
				}

				if self.module_conflicts {
					ui.collapsing("Module Conflicts", |ui| {
						TableBuilder::new(ui)
							.id_salt("Modules")
							.striped(true)
							.vscroll(true)
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.header(20.0, |mut header| {
								header.col(|ui| {
									ui.heading("ID");
								});
								header.col(|ui| {
									ui.heading("Name");
								});
								header.col(|ui| {
									ui.heading("Conflict Name");
								});
								header.col(|ui| {
									ui.heading("Conflict Source");
								});
							})
							.body(|mut body| {
								for (id, module) in &user_data.modules {
									if let Some(dma_modules) =
										self.dma_data.modules.uploaded_modules.get(id)
									{
										for dma_module in dma_modules {
											body.row(20.0, |mut row| {
												row.col(|ui| {
													ui.label(format!("{id}"));
												});
												row.col(|ui| {
													ui.label(
														module.name_en.clone().unwrap_or(
															module.name.clone().unwrap_or(
																module
																	.name_jp
																	.clone()
																	.unwrap_or_default(),
															),
														),
													);
												});
												row.col(|ui| {
													ui.label(
														dma_module
															.module
															.name_en
															.clone()
															.unwrap_or(
																dma_module
																	.module
																	.name
																	.clone()
																	.unwrap_or(
																		dma_module
																			.module
																			.name_jp
																			.clone()
																			.unwrap_or_default(),
																	),
															),
													);
												});
												row.col(|ui| {
													if let Some(post) = &dma_module.post {
														if let Some(post) =
															self.dma_data.modules.posts.get(post)
														{
															ui.hyperlink_to(
																&post.name,
																format!(
																	"https://divamodarchive.com/post/{}",
																	post.id
																),
															);
														} else {
															ui.label("Unknown");
														}
													} else {
														ui.label("MM+");
													}
												});
											});
										}
									}
								}
							});
					});
				}

				if self.module_reservation_conflicts {
					ui.collapsing("Module Reservation Conflicts", |ui| {
						TableBuilder::new(ui)
							.id_salt("ModuleReservations")
							.striped(true)
							.vscroll(true)
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.header(20.0, |mut header| {
								header.col(|ui| {
									ui.heading("ID");
								});
								header.col(|ui| {
									ui.heading("Name");
								});
								header.col(|ui| {
									ui.heading("Conflict Label");
								});
								header.col(|ui| {
									ui.heading("Conflict User");
								});
							})
							.body(|mut body| {
								for (id, module) in &user_data.modules {
									if let Some(reservation) =
										self.dma_data.modules.reserved_modules.get(id)
									{
										body.row(20.0, |mut row| {
											row.col(|ui| {
												ui.label(format!("{id}"));
											});
											row.col(|ui| {
												ui.label(module.name_en.clone().unwrap_or(
													module.name.clone().unwrap_or(
														module.name_jp.clone().unwrap_or_default(),
													),
												));
											});
											row.col(|ui| {
												if let Some(label) = &reservation.label {
													ui.label(label);
												}
											});
											row.col(|ui| {
												if let Some(user) = self
													.dma_data
													.modules
													.users
													.get(&reservation.user)
												{
													ui.label(&user.display_name);
												} else {
													ui.label("Unknown");
												}
											});
										});
									}
								}
							});
					});
				}

				if self.module_cos_reservation_conflicts {
					ui.collapsing("Costume Reservation Conflicts", |ui| {
						TableBuilder::new(ui)
							.id_salt("ModuleCosReservations")
							.striped(true)
							.vscroll(true)
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.header(20.0, |mut header| {
								header.col(|ui| {
									ui.heading("ID");
								});
								header.col(|ui| {
									ui.heading("Name");
								});
								header.col(|ui| {
									ui.heading("Conflict Label");
								});
								header.col(|ui| {
									ui.heading("Conflict User");
								});
							})
							.body(|mut body| {
								for (id, module) in &user_data.modules {
									if let Some(chara) =
										self.dma_data.modules.reserved_costumes.get(&module.chara)
									{
										if let Some(reservation) = chara.get(&module.cos.id) {
											body.row(20.0, |mut row| {
												row.col(|ui| {
													ui.label(format!("{id}"));
												});
												row.col(|ui| {
													ui.label(
														module.name_en.clone().unwrap_or(
															module.name.clone().unwrap_or(
																module
																	.name_jp
																	.clone()
																	.unwrap_or_default(),
															),
														),
													);
												});
												row.col(|ui| {
													if let Some(label) = &reservation.label {
														ui.label(label);
													}
												});
												row.col(|ui| {
													if let Some(user) = self
														.dma_data
														.modules
														.users
														.get(&reservation.user)
													{
														ui.label(&user.display_name);
													} else {
														ui.label("Unknown");
													}
												});
											});
										}
									}
								}
							});
					});
				}

				if self.cstm_item_conflicts {
					ui.collapsing("Cstm Item Conflicts", |ui| {
						TableBuilder::new(ui)
							.id_salt("CstmItems")
							.striped(true)
							.vscroll(true)
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.header(20.0, |mut header| {
								header.col(|ui| {
									ui.heading("ID");
								});
								header.col(|ui| {
									ui.heading("Name");
								});
								header.col(|ui| {
									ui.heading("Conflict Name");
								});
								header.col(|ui| {
									ui.heading("Conflict Source");
								});
							})
							.body(|mut body| {
								for (id, cstm_item) in &user_data.cstm_items {
									if let Some(dma_cstm_items) =
										self.dma_data.cstm_items.uploaded_cstm_items.get(id)
									{
										for dma_cstm_item in dma_cstm_items {
											body.row(20.0, |mut row| {
												row.col(|ui| {
													ui.label(format!("{id}"));
												});
												row.col(|ui| {
													ui.label(
														cstm_item.name_en.clone().unwrap_or(
															cstm_item.name.clone().unwrap_or(
																cstm_item
																	.name_jp
																	.clone()
																	.unwrap_or_default(),
															),
														),
													);
												});
												row.col(|ui| {
													ui.label(
														dma_cstm_item
															.cstm_item
															.name_en
															.clone()
															.unwrap_or(
																dma_cstm_item
																	.cstm_item
																	.name
																	.clone()
																	.unwrap_or(
																		dma_cstm_item
																			.cstm_item
																			.name_jp
																			.clone()
																			.unwrap_or_default(),
																	),
															),
													);
												});
												row.col(|ui| {
													if let Some(post) = &dma_cstm_item.post {
														if let Some(post) =
															self.dma_data.cstm_items.posts.get(post)
														{
															ui.hyperlink_to(
																&post.name,
																format!(
																	"https://divamodarchive.com/post/{}",
																	post.id
																),
															);
														} else {
															ui.label("Unknown");
														}
													} else {
														ui.label("MM+");
													}
												});
											});
										}
									}
								}
							});
					});
				}

				if self.cstm_item_reservation_conflicts {
					ui.collapsing("Cstm Item Reservation Conflicts", |ui| {
						TableBuilder::new(ui)
							.id_salt("CstmItemReservations")
							.striped(true)
							.vscroll(true)
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.column(Column::auto().resizable(true))
							.header(20.0, |mut header| {
								header.col(|ui| {
									ui.heading("ID");
								});
								header.col(|ui| {
									ui.heading("Name");
								});
								header.col(|ui| {
									ui.heading("Conflict Label");
								});
								header.col(|ui| {
									ui.heading("Conflict User");
								});
							})
							.body(|mut body| {
								for (id, cstm_item) in &user_data.cstm_items {
									if let Some(reservation) =
										self.dma_data.cstm_items.reserved_cstm_items.get(id)
									{
										body.row(20.0, |mut row| {
											row.col(|ui| {
												ui.label(format!("{id}"));
											});
											row.col(|ui| {
												ui.label(
													cstm_item.name_en.clone().unwrap_or(
														cstm_item.name.clone().unwrap_or(
															cstm_item
																.name_jp
																.clone()
																.unwrap_or_default(),
														),
													),
												);
											});
											row.col(|ui| {
												if let Some(label) = &reservation.label {
													ui.label(label);
												}
											});
											row.col(|ui| {
												if let Some(user) = self
													.dma_data
													.cstm_items
													.users
													.get(&reservation.user)
												{
													ui.label(&user.display_name);
												} else {
													ui.label("Unknown");
												}
											});
										});
									}
								}
							});
					});
				}

				if self.spr_set_conflicts {
					db_entry_table(
						ui,
						"Sprite Set",
						&user_data.spr_sets,
						&self.dma_data.spr_sets,
					);
				}

				if self.sprite_conflicts {
					db_entry_table(ui, "Sprite", &user_data.sprites, &self.dma_data.sprites);
				}

				if self.aet_set_conflicts {
					db_entry_table(ui, "Aet Set", &user_data.aet_sets, &self.dma_data.aet_sets);
				}

				if self.aet_scene_conflicts {
					db_entry_table(
						ui,
						"Aet Scene",
						&user_data.aet_scenes,
						&self.dma_data.aet_scenes,
					);
				}

				if self.objset_conflicts {
					db_entry_table(ui, "Objset", &user_data.objsets, &self.dma_data.objsets);
				}

				if self.texture_conflicts {
					db_entry_table(ui, "Texture", &user_data.textures, &self.dma_data.textures);
				}
			}
		});
	}
}

#[tokio::main]
async fn main() {
	let (pvs, modules, cstm_items, spr_sets, sprites, aet_sets, aet_scenes, objsets, textures) = tokio::join!(
		reqwest::get("https://divamodarchive.com/api/v1/ids/all_pvs"),
		reqwest::get("https://divamodarchive.com/api/v1/ids/all_modules"),
		reqwest::get("https://divamodarchive.com/api/v1/ids/all_cstm_items"),
		reqwest::get("https://divamodarchive.com/api/v1/ids/all_sprite_sets"),
		reqwest::get("https://divamodarchive.com/api/v1/ids/all_sprites"),
		reqwest::get("https://divamodarchive.com/api/v1/ids/all_aet_sets"),
		reqwest::get("https://divamodarchive.com/api/v1/ids/all_aet_scenes"),
		reqwest::get("https://divamodarchive.com/api/v1/ids/all_objsets"),
		reqwest::get("https://divamodarchive.com/api/v1/ids/all_textures"),
	);
	let (pvs, modules, cstm_items, spr_sets, sprites, aet_sets, aet_scenes, objsets, textures) = tokio::join!(
		pvs.unwrap().json::<AllPvs>(),
		modules.unwrap().json::<AllModules>(),
		cstm_items.unwrap().json::<AllCstmItems>(),
		spr_sets.unwrap().json::<AllDbEntries>(),
		sprites.unwrap().json::<AllDbEntries>(),
		aet_sets.unwrap().json::<AllDbEntries>(),
		aet_scenes.unwrap().json::<AllDbEntries>(),
		objsets.unwrap().json::<AllDbEntries>(),
		textures.unwrap().json::<AllDbEntries>(),
	);

	let (pvs, modules, cstm_items, spr_sets, sprites, aet_sets, aet_scenes, objsets, textures) = (
		pvs.unwrap(),
		modules.unwrap(),
		cstm_items.unwrap(),
		spr_sets.unwrap(),
		sprites.unwrap(),
		aet_sets.unwrap(),
		aet_scenes.unwrap(),
		objsets.unwrap(),
		textures.unwrap(),
	);

	let app = AppData {
		dma_data: DmaData {
			pvs,
			modules,
			cstm_items,
			spr_sets,
			sprites,
			aet_sets,
			aet_scenes,
			objsets,
			textures,
		},
		user_data: None,

		pv_conflicts: false,
		pv_reservation_conflicts: false,
		module_conflicts: false,
		module_reservation_conflicts: false,
		module_cos_reservation_conflicts: false,
		cstm_item_conflicts: false,
		cstm_item_reservation_conflicts: false,
		spr_set_conflicts: false,
		sprite_conflicts: false,
		aet_set_conflicts: false,
		aet_scene_conflicts: false,
		objset_conflicts: false,
		texture_conflicts: false,
	};

	eframe::run_native(
		"DMA Conflict Checker",
		eframe::NativeOptions::default(),
		Box::new(|_cc| Ok(Box::new(app))),
	)
	.unwrap();
}
