import { StrictMode, Suspense } from "react";
import { Provider } from "@conformal/plugin";
import infos from "./mock_infos.ts";
import ModeProvider from "./ModeProvider.tsx";
import App from "./App.tsx";

export const RootProviders = ({ children }: { children: React.ReactNode }) => (
  <StrictMode>
    <Provider mockInfos={infos}>
      <ModeProvider>
        <Suspense fallback={<></>}>{children}</Suspense>
      </ModeProvider>
    </Provider>
  </StrictMode>
);

export const Root = () => (
  <RootProviders>
    <App />
  </RootProviders>
);

export default Root;
