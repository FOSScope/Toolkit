import { useRef } from "react";

interface Props {
  onChangPage: (page: boolean) => void;
  user: string;
  setUser: (user: string) => void;
  token: string;
  setToken: (token: string) => void;
  deepseek: string;
  setDeepseek: (deepseek: string) => void;
}

function Login(props: Props) {
  const input1Ref = useRef<HTMLInputElement>(null);
  const input2Ref = useRef<HTMLInputElement>(null);
  const input3Ref = useRef<HTMLInputElement>(null);

  //user,token,deepseek保存，同时切换页面。往后端传这三个数据可以在此处传
  const handleSave = () => {
    if (input1Ref.current && input2Ref.current && input3Ref.current) {
      if (input1Ref.current.value != "") {
        props.setUser(input1Ref.current.value);
      }
      if (input2Ref.current.value != "") {
        props.setToken(input2Ref.current.value);
      }
      if (input3Ref.current.value != "") {
        props.setDeepseek(input3Ref.current.value);
      }
    }
    props.onChangPage(true);
  };

  return (
    <>
      <div className="container my-5">
        <div className="p-5 bg-light rounded-3">
          <h1 className="text-center">FOSScope Toolkit</h1>
          <p className="text-center">为开源观察贡献者设计的工具箱</p>
        </div>
      </div>
      <div className="container my-5">
        <div className="row justify-content-center">
          <div className="col-md-6 half-width-container p-5">
            <p className="text-secondary">信息可以在稍后更改</p>
            <div className="input-group input-group-lg">
              <span className="input-group-text" id="inputGroup-sizing-lg">
                Github用户名
              </span>
              <input
                type="text"
                className="form-control"
                aria-label="Sizing example input"
                aria-describedby="inputGroup-sizing-lg"
                ref={input1Ref}
                placeholder={props.user}
              ></input>
            </div>
            <div className="input-group input-group-lg my-3">
              <span className="input-group-text" id="inputGroup-sizing-lg">
                Github Token
              </span>
              <input
                type="text"
                className="form-control"
                aria-label="Sizing example input"
                aria-describedby="inputGroup-sizing-lg"
                ref={input2Ref}
                placeholder={props.token}
              ></input>
            </div>
            <div className="input-group input-group-lg my-3">
              <span className="input-group-text" id="inputGroup-sizing-lg">
                DeepSeek API Key
              </span>
              <input
                type="text"
                className="form-control"
                aria-label="Sizing example input"
                aria-describedby="inputGroup-sizing-lg"
                ref={input3Ref}
                placeholder={props.deepseek}
              ></input>
            </div>
            <div className="d-flex justify-content-center my-5">
              <button
                type="button"
                className="btn btn-success btn-lg"
                onClick={handleSave}
              >
                保存并进入主页
              </button>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}

export default Login;
