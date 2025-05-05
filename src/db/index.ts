import Database from '@tauri-apps/plugin-sql';

const db = await Database.load('sqlite:mimosa.db');

export default function getDb() {
  return db;
}