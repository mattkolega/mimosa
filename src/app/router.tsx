import { BrowserRouter, Routes, Route } from "react-router";

import BaseLayout from "@/components/base-layout";
import Home from "@/app/routes/home";

const AppRouter = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<BaseLayout />}>
          <Route index element={<Home />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default AppRouter;