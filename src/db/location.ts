import { invoke } from '@tauri-apps/api/core';

export type Location = {
  id: number;
  path: string;
}

export async function getAllLocations(): Promise<Location[]> {
  return invoke<Location[]>('get_all_locations')
    .then((result) => result)
    .catch((error) => {
      console.error(error)
      return [];
    });
}

export async function addLocation(path: string): Promise<number> {
  return invoke<number>('add_location', {path: path})
    .then((result) => result)
    .catch((error) => {
      console.error(error)
      return 0;
    });
}

export async function deleteLocation(path: string): Promise<number> {
  return invoke<number>('delete_location', {path: path})
    .then((result) => result)
    .catch((error) => {
      console.error(error)
      return 0;
    });
}