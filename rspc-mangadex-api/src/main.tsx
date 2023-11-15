
import { createRoot } from 'react-dom/client'
import { Routes } from '@generouted/react-router'
import { ChakraProvider } from "@chakra-ui/react";

createRoot(document.getElementById('root')!).render(
  <ChakraProvider>
    <Routes />
  </ChakraProvider>
);
