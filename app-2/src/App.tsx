import React, { useEffect, useMemo, useState } from "react";
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
    className: "plane-icon",
    iconSize: [30, 30],
    iconAnchor: [15, 15],
  });
};

function App() {
  const [time, setTime] = useState<Date>();
  const [planes, setPlanes] = useState<Plane[]>([]);

  const fetchPlanes = async () => {
    try {
      const endDate = time == null ? new Date() : time;
      const startDate = new Date(endDate.getTime() - 600_000);

      const response = await fetch("http://localhost:3000/planes", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          start: startDate.toISOString(),
          end: endDate.toISOString(),
        }),
      });
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const data: Plane[] = await response.json();

      console.log(data);

      setPlanes(data);
    } catch (err: any) {
      console.log(err);
    }
  };

  useEffect(() => {
    fetchPlanes();

    const intervalId = setInterval(() => {
      fetchPlanes();
    }, 1000);

    return () => clearInterval(intervalId);
  }, []);

  const markers = useMemo(() => {
    return planes.map((p) => <MapMarker {...p} key={p.hex_ident} />);
  }, [planes]);

  return (
    <div className="App">
      <header className="App-header">
        <MapContainer center={DENVER} zoom={13}>
          <TileLayer
            attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
            url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
          />
          {markers}
        </MapContainer>
      </header>
    </div>
  );
}

const MapMarker = React.memo(
  ({
    latitude,
    longitude,
    callsign,
    hex_ident,
    track,
  }: {
    latitude: number;
    longitude: number;
    track: number;
    callsign: string;
    hex_ident: string;
  }) => {
    // function MapMarker({p}: {p: Plane}) {
    const position: any = useMemo(
      () => [latitude, longitude],
      [latitude, longitude]
    );
    const icon = useMemo(() => createPlaneIcon(track), [track]);

    console.log("rending map marker");

    return (
      <Marker position={position} icon={icon}>
        <MyPopup callsign={callsign} hex_ident={hex_ident}></MyPopup>
        {/* <Popup autoClose={false}>
        <div>Callsign: {callsign}</div>
        <div>Hex Ident: {hex_ident}</div>
      </Popup> */}
      </Marker>
    );
  }
);

const MyPopup = React.memo(
  ({ callsign, hex_ident }: { callsign: string; hex_ident: string }) => {
    console.log("rendering popup");
    return (
      <Popup autoClose={false}>
        <div>Callsign: {callsign}</div>
        <div>Hex Ident: {hex_ident}</div>
      </Popup>
    );
  }
);

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
