import getDb from "@/db";

export type Location = {
  id: number;
  path: string;
}

export async function getAllLocations(): Promise<Location[]> {
  return await getDb().select<Location[]>("SELECT * FROM location");
}

export async function addLocation(path: string): Promise<number> {
  const result = await getDb().execute(
    "INSERT INTO location (path) VALUES ($1)",
    [path],
  );

  return result.lastInsertId ?? 0;
}

export async function deleteLocation(path: string): Promise<number> {
  const result = await getDb().execute(
    "DELETE FROM location WHERE path = $1",
    [path],
  );

  return result.rowsAffected;
}