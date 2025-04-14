import { BrowserRouter, Routes, Route } from "react-router";

import Home from "@/app/routes/home";
import Library from "@/app/routes/library";
import Settings from "@/app/routes/settings";

import BaseLayout from "@/components/base-layout";

const AppRouter = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<BaseLayout />}>
          <Route index element={<Home />} />
          <Route path="/library" element={<Library />} />
          <Route path="/settings" element={<Settings />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default AppRouter;