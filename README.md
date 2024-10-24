# Trustify - Blockchain-based Escrow Platform

Trustify is a decentralized escrow platform designed to provide a secure and transparent environment for transactions between buyers and sellers. Built using blockchain technology, it ensures that funds are only released once the agreed conditions are met, safeguarding both parties from fraud or disputes.

[![GitHub stars](https://img.shields.io/github/stars/kiavin/Trustify.svg?style=social&label=Star)](https://github.com/kiavin/Trustify/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/kiavin/Trustify.svg?style=social&label=Fork)](https://github.com/kiavin/Trustify/network)
[![GitHub issues](https://img.shields.io/github/issues/kiavin/Trustify.svg)](https://github.com/kiavin/Trustify/issues)

## Table of Contents
- [Live Project](#live-project)
- [Project Overview](#project-overview)
- [Key Features](#key-features)
- [Tech Stack](#tech-stack)
- [How It Works](#how-it-works)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [Future Features](#future-features)
- [Contributors](#contributors)
- [License](#license)

## Live Project
[Trustify Escrow Platform](https://kv2lk-baaaa-aaaal-qnaoq-cai.icp0.io/)

## Project Overview

Trustify is built to reduce risks in online transactions by implementing an escrow service that locks funds until the terms of a deal are met. With blockchain technology at its core, Trustify ensures:

- **Security**: Transactions are recorded on a tamper-proof blockchain.
- **Transparency**: Both buyers and sellers can track the progress of their transactions in real-time.
- **Decentralization**: Trustify operates without intermediaries, relying on smart contracts to mediate all operations.

## Key Features

- **Blockchain-based Escrow**: Funds are securely held in escrow until the conditions of the agreement are satisfied.
- **Rust Backend**: The core logic is implemented in Rust for performance, security, and reliability.
- **Vue.js Frontend**: A user-friendly interface built using Vue.js for a seamless experience.
- **ICP Hosting**: Trustify is hosted on the Internet Computer Protocol (ICP), ensuring decentralized infrastructure and scalability.

## Tech Stack

### Frontend:
- **Vue.js**: For building responsive and dynamic user interfaces.
- **Tailwind CSS**: For responsive, modern design and layout.

### Backend:
- **Rust**: The smart contract logic is built in Rust for secure and fast execution.
- **Internet Computer Protocol (ICP)**: For decentralized and scalable hosting.

## How It Works

1. **Create Escrow**: A buyer creates an escrow by defining the terms of the agreement.
2. **Seller Agreement**: The seller reviews and agrees to the terms.
3. **Fund Escrow**: The buyer deposits the required amount into the escrow.
4. **Complete Transaction**: The seller fulfills their end of the deal.
5. **Release Funds**: Once the buyer confirms receipt, funds are automatically released to the seller.
6. **Dispute Resolution**: If there are any disputes, the smart contract handles them according to predefined rules.

## Project Structure

### Backend (Rust)
- **Smart Contracts**: The core logic for escrow operations—funding, agreement, and release—is written in Rust, ensuring robust and secure transaction management.

### Frontend (Vue.js)
- **Responsive UI**: The frontend is built with Vue.js, offering a clean and intuitive user experience.
- **Real-time Escrow Tracking**: Users can monitor the status of their escrow transactions in real-time through the platform’s interface.

## Getting Started

### Prerequisites
- Node.js
- Rust
- dfx (for ICP)

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/kiavin/Trustify.git
   ```

2. **Navigate to the project directory:**
   ```bash
   cd trustify
   ```

3. **Install dependencies:**

   For the frontend:
   ```bash
   cd frontend
   npm install
   ```

   For the backend:
   ```bash
   cd backend
   cargo build
   ```

4. **Start the development server:**

   For the frontend:
   ```bash
   npm run dev
   ```

   For the backend:
   ```bash
   dfx start
   ```

5. **Deploy the Canister (Backend):**
   Follow the official [ICP deployment guide](https://sdk.dfinity.org/docs/developers-guide/deploy-app.html) to deploy your canister on the Internet Computer.

## Future Features

- **Multicurrency Support**: Enabling the use of various cryptocurrencies.
- **Dispute Arbitration**: Decentralized arbitration for resolving disputes between buyers and sellers.
- **Analytics Dashboard**: Transaction insights and performance metrics.
- **Mobile App**: A companion mobile app for convenient transactions on the go.

## Contributors

Thanks to these wonderful people who contribute to this project:

- [kiavin](https://github.com/kiavin) - 16 contributions
- [Vickey-m-victor](https://github.com/Vickey-m-victor) - 8 contributions
- [codetheuri](https://github.com/codetheuri) - 5 contributions

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.

---

Trustify – Empowering trust in every transaction with the security of blockchain. Check out the live platform [here](https://kv2lk-baaaa-aaaal-qnaoq-cai.icp0.io/) https://kv2lk-baaaa-aaaal-qnaoq-cai.icp0.io/ and start securing your deals today!
