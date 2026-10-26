import { createRoot } from 'react-dom/client';
import App from './components/App.tsx';

import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';
import './styles/index.css';

createRoot(document.getElementById('root')!).render(<App />);
