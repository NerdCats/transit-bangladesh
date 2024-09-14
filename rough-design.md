`# Technical Design Document: Bus Route Map Matching CLI Tool (in Rust)

## Overview
This Rust CLI tool will take a GeoJSON file containing bus routes in Dhaka in the form of LineStrings, map match those routes using the Valhalla routing engine, and output a new GeoJSON file with map-matched LineStrings. The tool will download a Valhalla Docker image, initialize a Docker container with the road network of Bangladesh from OpenStreetMap, communicate with Valhalla for map matching, and finally clean up resources after the operation is complete.

---

## Requirements

### Functional Requirements
1. The tool accepts a GeoJSON file with a FeatureCollection of LineStrings representing bus routes in Dhaka.
2. Each LineString must be map matched to the closest routes in the Valhalla routing engine using OpenStreetMap road data for Bangladesh.
3. The tool generates a new GeoJSON file with the same FeatureCollection structure, but with map-matched LineStrings.
4. The tool must handle the process of downloading and running Valhalla inside a Docker container.
5. Valhalla should automatically shut down and the Docker container should be cleaned up after map matching is complete.

### Non-Functional Requirements
1. The tool must be easy to run via a command-line interface.
2. The communication with the Valhalla engine must be efficient, minimizing network or resource overhead.
3. The tool should handle errors such as:
    - Invalid GeoJSON files.
    - Network issues when downloading the Docker image or OpenStreetMap data.
    - Failure to start or stop the Docker container.
4. The tool should be able to handle moderately large input GeoJSON files with multiple LineStrings.

---

## Sequence Diagram

```plaintext
User -> CLI Tool: Provide GeoJSON Input
CLI Tool -> Docker: Download Valhalla Docker Image
CLI Tool -> Docker: Start Valhalla Container
Docker -> Valhalla: Initialize Routing Engine with OSM Data
CLI Tool -> Valhalla: Send LineStrings for Map Matching
Valhalla -> CLI Tool: Return Map Matched LineStrings
CLI Tool -> File System: Write New GeoJSON Output
CLI Tool -> Docker: Stop Valhalla Container
```

---

## Command Line Interface Design

### Usage
```bash
transit-bd map-match <input-file> <output-file> [OPTIONS]
```

### Positional Arguments:
- `<input-file>`: Path to the input GeoJSON file that contains the FeatureCollection of bus routes (LineStrings).
- `<output-file>`: Path to the output GeoJSON file that will contain map-matched routes.

### Options:
- `--docker-image <image>`: The Docker image of Valhalla to use. Defaults to `valhalla-docker`.
- `--osm-data <path>`: Path to the OpenStreetMap data for Bangladesh. If not specified, the tool downloads it automatically.
- `--valhalla-port <port>`: The port on which Valhalla will run. Defaults to `8002`.
- `--cleanup`: Automatically removes the Docker container after the process is complete. (Enabled by default)
- `--no-cleanup`: If passed, the Docker container is not removed after the process.
- `--help`: Shows the help menu.

### Example Usage:
```bash
transit-bd map-match bus_routes.geojson map_matched_routes.geojson --docker-image valhalla-docker --osm-data ./bangladesh.osm.pbf
```


## Future Considerations
- Option to keep the Valhalla container running for multiple map matching operations.
