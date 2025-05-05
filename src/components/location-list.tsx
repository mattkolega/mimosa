import { useEffect, useRef, useState } from "react";
import { Plus, X } from "lucide-react";
import { open } from '@tauri-apps/plugin-dialog';

import { Button } from "@/components/ui/button";
import { type Location, getAllLocations, addLocation, deleteLocation } from "@/db/location";
import { cn } from "@/lib/utils";

const LocationList = () => {
  const [locations, setLocations] = useState<Location[]>([]);
  const [selectedPath, setSelectedPath] = useState<Location | null>(null);

  const listContainerRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    // Grab stored location data
    const fetchLocations = async () => {
      const data = await getAllLocations();
      setLocations(data);
    }

    fetchLocations();

    // Handle click outside of component to de-select path
    const handleClickOutside = (event: MouseEvent) => {
      if (listContainerRef.current && !listContainerRef.current.contains(event.target as Node)) {
        setSelectedPath(null);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);

    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, []);

  const addPath = async () => {
    const path = await open({ directory: true });
    if (path === null) return;

    if (locations.find(item => path === item.path)) return;  // Ensure new path isn't a duplicate

    const newId = await addLocation(path);
    if (newId === 0) return;

    setLocations(
      [...locations, { id: newId, path: path }]
    );
  }

  const deletePath = async () => {
    if (!selectedPath) return;

    const rowsAffected = await deleteLocation(selectedPath.path);
    if (rowsAffected === 0) return;

    setLocations(
      locations.filter(item => item.id !== selectedPath.id)
    );
    setSelectedPath(null);
  }

  const blankRowCount = locations.length < 5 ? 5 - locations.length : 0;
  const blankRows = Array.from({ length: blankRowCount });

  return (
    <div ref={listContainerRef} className="flex flex-col w-lg border">
      <div className="flex flex-col h-30 w-full overflow-y-scroll">
        {locations.map((item) =>
          <div
            onClick={() => setSelectedPath(item)} key={item.id}
            className={cn("h-6 px-1 w-full even:bg-accent", (selectedPath && item.id === selectedPath.id) && "bg-blue-300 even:bg-blue-300")}
          >
            {item.path}
          </div>
        )}
        {blankRows.map((_, index) =>
          <div key={index} className="h-6 w-full even:bg-accent">{/* Placeholder */}</div>
        )}
      </div>
      <div className="flex flex-row h-8 w-full px-1 border-t justify-end items-center gap-0.5">
        <Button onClick={addPath} variant="ghost" className="w-6 h-6 rounded-sm">
          <Plus strokeWidth={1.5} className="size-6"/>
        </Button>
        <Button onClick={deletePath} variant="ghost" className="w-6 h-6 rounded-sm">
          <X strokeWidth={1.5} className="size-6"/>
        </Button>
      </div>
    </div>
  );
}

export default LocationList;