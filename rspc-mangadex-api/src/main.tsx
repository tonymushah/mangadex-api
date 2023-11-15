
import { createRoot } from 'react-dom/client'
import { Routes } from '@generouted/react-router'
import { ChakraProvider } from "@chakra-ui/react";
import IPCProvider from './lib/ipc';

createRoot(document.getElementById('root')!).render(
  <ChakraProvider>
    <IPCProvider>
      <Routes />
    </IPCProvider>
  </ChakraProvider>
);
