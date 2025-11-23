/*
 * Original Magisk Copyright:
 * Copyright 2017 - 2025, John Wu (@topjohnwu)
 *
 * RAFAELIA Framework Additions:
 * Copyright 2025, Rafael Melo Reis (rafaelmeloreisnovo)
 * Instituto Rafael - CientiEspiritual Philosophy
 *
 * This file is part of Magisk_Rafaelia, a derivative work of Magisk.
 * 
 * Magisk_Rafaelia is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 *
 * ---
 * RAFAELIA PHILOSOPHY (Aspirational Commentary - Not Part of License):
 * 
 * Sacred Cycle: VAZIO → VERBO → CHEIO → RETRO (EMPTY → ACTION → FULL → FEEDBACK)
 * Motto: "Amor, Luz e Coerência" (Love, Light and Coherence)
 * Ethica[8]: Transparency, Accountability, Fairness, Privacy, Security,
 *            Reliability, Safety, Sustainability
 * ---
 */

use crate::daemon::{
    AID_APP_END, AID_APP_START, AID_ROOT, AID_SHELL, MagiskD, to_app_id, to_user_id,
};
use crate::db::DbArg::Integer;
use crate::db::{MultiuserMode, RootAccess, SqlTable, SqliteResult, SqliteReturn};
use crate::ffi::{DbValues, SuPolicy};
use base::ResultExt;

impl Default for SuPolicy {
    fn default() -> Self {
        SuPolicy::Query
    }
}

#[derive(Default)]
pub struct RootSettings {
    pub policy: SuPolicy,
    pub log: bool,
    pub notify: bool,
}

impl SqlTable for RootSettings {
    fn on_row(&mut self, columns: &[String], values: &DbValues) {
        for (i, column) in columns.iter().enumerate() {
            let val = values.get_int(i as i32);
            match column.as_str() {
                "policy" => self.policy.repr = val,
                "logging" => self.log = val != 0,
                "notification" => self.notify = val != 0,
                _ => {}
            }
        }
    }
}

struct UidList(Vec<i32>);

impl SqlTable for UidList {
    fn on_row(&mut self, _: &[String], values: &DbValues) {
        self.0.push(values.get_int(0));
    }
}

impl MagiskD {
    pub fn get_root_settings(&self, uid: i32, settings: &mut RootSettings) -> SqliteResult<()> {
        self.db_exec_with_rows(
            "SELECT policy, logging, notification FROM policies \
             WHERE uid=? AND (until=0 OR until>strftime('%s', 'now'))",
            &[Integer(uid as i64)],
            settings,
        )
        .sql_result()
    }

    pub fn prune_su_access(&self) {
        let mut list = UidList(Vec::new());
        if self
            .db_exec_with_rows("SELECT uid FROM policies", &[], &mut list)
            .sql_result()
            .log()
            .is_err()
        {
            return;
        }

        let app_list = self.get_app_no_list();
        let mut rm_uids = Vec::new();

        for uid in list.0 {
            let app_id = to_app_id(uid);
            if (AID_APP_START..=AID_APP_END).contains(&app_id) {
                let app_no = app_id - AID_APP_START;
                if !app_list.contains(app_no as usize) {
                    // The app_id is no longer installed
                    rm_uids.push(uid);
                }
            }
        }

        for uid in rm_uids {
            self.db_exec("DELETE FROM policies WHERE uid=?", &[Integer(uid as i64)]);
        }
    }

    pub fn uid_granted_root(&self, mut uid: i32) -> bool {
        if uid == AID_ROOT {
            return true;
        }

        let cfg = match self.get_db_settings().log() {
            Ok(cfg) => cfg,
            Err(_) => return false,
        };

        // Check user root access settings
        match cfg.root_access {
            RootAccess::Disabled => return false,
            RootAccess::AppsOnly => {
                if uid == AID_SHELL {
                    return false;
                }
            }
            RootAccess::AdbOnly => {
                if uid != AID_SHELL {
                    return false;
                }
            }
            _ => {}
        }

        // Check multiuser settings
        match cfg.multiuser_mode {
            MultiuserMode::OwnerOnly => {
                if to_user_id(uid) != 0 {
                    return false;
                }
            }
            MultiuserMode::OwnerManaged => uid = to_app_id(uid),
            _ => {}
        }

        let mut granted = false;
        let mut output_fn =
            |_: &[String], values: &DbValues| granted = values.get_int(0) == SuPolicy::Allow.repr;
        self.db_exec_with_rows(
            "SELECT policy FROM policies WHERE uid=? AND (until=0 OR until>strftime('%s', 'now'))",
            &[Integer(uid as i64)],
            &mut output_fn,
        );

        granted
    }
}
