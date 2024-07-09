import { useState } from "react";

import Login from "./components/Login";
import MainPage from "./components/MainPage";

import "./App.css";

//github username,token, deepseek储存在本层，如有需要，通过props传递给子组件，也可传递给后端
//Login、MainPage组件也有这三个，也可传递给后端，看怎么方便
function App() {
  const [isLogin, setIsLogin] = useState(false);
  const [githubUser, setGithubUser] = useState("");
  const [githubToken, setGithubToken] = useState("");
  const [deepseek, setDeepseek] = useState("");

  //在登录页和主界面中切换
  const handlePageChange = (page: boolean) => {
    setIsLogin(page);
  };

  //改github username
  const handleUserChange = (user: string) => {
    setGithubUser(user);
  };

  //改github token
  const handleTokenChange = (token: string) => {
    setGithubToken(token);
  };

  // 改deepseek API key
  const handleDeepseekChange = (deepseek: string) => {
    setDeepseek(deepseek);
  };

  return (
    <>
      {isLogin ? (
        //进主界面
        <MainPage
          deepseek={deepseek}
          onChangPage={handlePageChange}
          user={githubUser}
          token={githubToken}
        ></MainPage>
      ) : (
        //进入登录界面
        <Login
          onChangPage={handlePageChange}
          user={githubUser}
          token={githubToken}
          setToken={handleTokenChange}
          setUser={handleUserChange}
          deepseek={deepseek}
          setDeepseek={handleDeepseekChange}
        ></Login>
      )}
    </>
  );
}

export default App;
