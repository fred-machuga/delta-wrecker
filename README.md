**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

# Delta Wrecker

**Delta Wrecker** is an orbital salvage game utilizing real mechanics to rescue satellites and clear space debris. Experience the challenge of running a one-ship salvage company: performing precision burns, intercepts, grappling, and towing targets to graveyard orbits or repair stations.

## Prerequisites

Before you begin, ensure you have the following installed:
- **Rust** (latest stable)
- **Python** (3.8+)
- **maturin** (for Rust-Python bindings)
- **Pygame** (for the game engine/visuals)

## Setup and Build Instructions

1. **Clone the repository:**
   ```bash
   git clone https://github.com/fred-machuga/delta-wrecker.git
   cd delta-wrecker
   ```

2. **Create and activate a Python virtual environment:**
   ```bash
   python -m venv venv
   # On Windows:
   .\venv\Scripts\activate
   # On macOS/Linux:
   source venv/bin/activate
   ```

3. **Install Python dependencies:**
   ```bash
   pip install maturin pygame
   ```

4. **Build the Rust extension using maturin:**
   ```bash
   maturin develop
   ```
   *(Note: This compiles the Rust core logic and correctly links it into your currently active Python virtual environment).*

5. **Run the application:**
   ```bash
   python main.py
   ```

**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.