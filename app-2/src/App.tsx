import React from "react";
import logo from "./logo.svg";
import "./App.css";
import { MapContainer, Marker, Popup, TileLayer, useMap } from "react-leaflet";
import { LatLngExpression } from "leaflet";
import 'leaflet/dist/leaflet.css';
import 'leaflet/dist/leaflet.js';

const DENVER: LatLngExpression = [39.76, -105.08];

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <MapContainer center={DENVER} zoom={13}>
          <TileLayer
            attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
            url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
          />
          <Marker position={DENVER}>
            <Popup>
              A pretty CSS3 popup. <br /> Easily customizable.
            </Popup>
          </Marker>
        </MapContainer>
      </header>
    </div>
  );
}

export default App;
