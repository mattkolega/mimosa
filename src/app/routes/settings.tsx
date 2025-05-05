import LocationList from "@/components/location-list";

const Settings = () => {
  return (
    <div className="py-1.5 px-5 h-full w-full">
      <h1 className="text-4xl font-bold mb-3">Settings</h1>
      <h2 className="text-xl mb-1">Music Sources</h2>
      <LocationList />
    </div>
  );
}

export default Settings;