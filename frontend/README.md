# InkTix Frontend

A modern, responsive web application for the InkTix cross-chain sports ticketing platform, built with Next.js, TypeScript, and Tailwind CSS.

## 🚀 Features

- **Modern UI/UX**: Beautiful, responsive design with smooth animations
- **Wallet Integration**: Polkadot wallet connection for Web3 functionality
- **Cross-Chain Ready**: Built to work with InkTix's XCM integration
- **Sports Events**: Browse and filter sports events across all chains
- **Responsive Design**: Mobile-first approach with Tailwind CSS
- **TypeScript**: Full type safety and better developer experience

## 🛠️ Tech Stack

- **Framework**: Next.js 14 with App Router
- **Language**: TypeScript
- **Styling**: Tailwind CSS with custom design system
- **Icons**: Lucide React
- **Web3**: Polkadot.js integration (planned)
- **State Management**: React hooks (Zustand planned for future)

## 📁 Project Structure

```
frontend/
├── src/
│   ├── app/                 # Next.js App Router pages
│   │   ├── connect/        # Wallet connection page
│   │   ├── events/         # Sports events page
│   │   ├── globals.css     # Global styles and Tailwind
│   │   ├── layout.tsx      # Root layout component
│   │   └── page.tsx        # Home page
│   ├── components/          # Reusable UI components
│   │   └── WalletConnect.tsx
│   ├── lib/                 # Utility libraries (future)
│   ├── types/               # TypeScript type definitions (future)
│   ├── hooks/               # Custom React hooks (future)
│   └── utils/               # Utility functions (future)
├── public/                  # Static assets
├── tailwind.config.js       # Tailwind CSS configuration
├── next.config.js           # Next.js configuration
├── tsconfig.json            # TypeScript configuration
└── package.json             # Dependencies and scripts
```

## 🚀 Getting Started

### Prerequisites

- Node.js 18+
- npm or yarn
- Polkadot wallet extension (Polkadot.js, Talisman, etc.)

### Installation

1. **Clone the repository**

   ```bash
   cd frontend
   ```

2. **Install dependencies**

   ```bash
   npm install
   ```

3. **Start development server**

   ```bash
   npm run dev
   ```

4. **Open your browser**
   Navigate to [http://localhost:3000](http://localhost:3000)

### Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run start` - Start production server
- `npm run lint` - Run ESLint

## 🎨 Design System

### Colors

- **Primary**: Blue shades for main actions and branding
- **Secondary**: Gray shades for text and backgrounds
- **Accent**: Yellow/Orange for highlights and CTAs

### Components

- **Buttons**: Primary, secondary, and accent variants
- **Cards**: Consistent card design with hover effects
- **Forms**: Styled input fields and form elements
- **Badges**: Status and category indicators

### Typography

- **Font**: Inter for body text, JetBrains Mono for code
- **Sizes**: Responsive typography scale
- **Weights**: 300, 400, 500, 600, 700

## 🔌 Wallet Integration

The frontend includes a comprehensive wallet connection system:

- **Supported Wallets**: Polkadot.js, Talisman, SubWallet, Nova
- **Connection Flow**: Secure wallet detection and connection
- **Account Management**: Multiple account support and switching
- **Error Handling**: User-friendly error messages and guidance

## 📱 Responsive Design

- **Mobile First**: Designed for mobile devices first
- **Breakpoints**: Tailwind's responsive breakpoints
- **Grid System**: Flexible grid layouts for all screen sizes
- **Touch Friendly**: Optimized for touch interactions

## 🔮 Future Enhancements

### Phase 1 (Current)

- ✅ Basic UI components and pages
- ✅ Wallet connection system
- ✅ Responsive design
- ✅ Event browsing interface

### Phase 2 (Planned)

- [ ] Smart contract integration
- [ ] Real-time event data
- [ ] Ticket purchasing flow
- [ ] User dashboard

### Phase 3 (Future)

- [ ] Cross-chain functionality
- [ ] Fantasy sports integration
- [ ] Advanced analytics
- [ ] Mobile app (PWA)

## 🧪 Testing

Currently using manual testing. Future plans include:

- [ ] Unit tests with Jest/React Testing Library
- [ ] Integration tests for wallet connection
- [ ] E2E tests with Playwright
- [ ] Visual regression testing

## 📦 Deployment

### Development

```bash
npm run dev
```

### Production Build

```bash
npm run build
npm run start
```

### Environment Variables

Create a `.env.local` file for environment-specific configuration:

```env
NEXT_PUBLIC_CONTRACT_ADDRESS=your_contract_address
NEXT_PUBLIC_NETWORK=polkadot
NEXT_PUBLIC_RPC_ENDPOINT=your_rpc_endpoint
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the same license as the main InkTix project.

## 🆘 Support

For support and questions:

- Check the main InkTix documentation
- Open an issue in the repository
- Contact the development team

---

**Built with ❤️ for the Polkadot ecosystem**

