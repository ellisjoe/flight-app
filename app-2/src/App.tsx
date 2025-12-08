import React, { useEffect, useState } from "react";
import logo from "./logo.svg";
import "./App.css";
import { MapContainer, Marker, Popup, TileLayer, useMap } from "react-leaflet";
import L, { Icon, LatLngExpression, DivIcon } from "leaflet";
import "leaflet/dist/leaflet.css";
import "leaflet/dist/leaflet.js";
import planeSvg from "./plane.svg";

const DENVER: LatLngExpression = [39.76, -105.08];

const createPlaneIcon = (rotation: number): DivIcon => {
  return L.divIcon({
    html: `<img src="${planeSvg}" style="width: 30px; height: 30px; transform: rotate(${rotation}deg);" />`,
    className: 'plane-icon',
    iconSize: [30, 30],
    iconAnchor: [15, 15],
  });
};

function App() {
  const [time, setTime] = useState<Date>();
  const [planes, setPlanes] = useState<Plane[]>([]);

  const fetchUsers = async () => {
    try {
      const endDate = time == null ? new Date() : time;
      const startDate = new Date(endDate.getTime() - 600_000)

      const response = await fetch("http://localhost:3000/planes", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          // start: startDate.toISOString(),
          end: endDate.toISOString(),
        }),
      });
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const data: Plane[] = await response.json();

      console.log(planes);

      setPlanes(data);
    } catch (err: any) {
      console.log(err);
    }
  };

  useEffect(() => {
    fetchUsers();

    const intervalId = setInterval(() => {
      fetchUsers();
    }, 1000); // 1000 milliseconds = 1 second

    return () => clearInterval(intervalId);
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <MapContainer center={DENVER} zoom={13}>
          <TileLayer
            attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
            url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
          />
          {planes.map((p) => (
            <Marker key={p.hex_ident} position={[p.latitude, p.longitude]} icon={createPlaneIcon(p.track)}>
              <Popup>
                {p.callsign}
              </Popup>
            </Marker>
          ))}
        </MapContainer>
      </header>
    </div>
  );
}

interface Plane {
  aircraft_id: string;
  hex_ident: string;
  generated_timestamp: string;
  logged_timestamp: string;
  callsign: string;
  altitude: number;
  ground_speed: number;
  track: number;
  latitude: number;
  longitude: number;
  vertical_rate: number;
  squawk: string;
}

export default App;
