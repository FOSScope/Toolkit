import React from "react";
import TransForkRepo from "./TransForkRepo";
import OrigForkRepo from "./OrigForkRepo";
import {
  MDBContainer,
  MDBCol,
  MDBCard,
  MDBCardBody,
  MDBCardTitle,
  MDBCardText,
  MDBBtn,
  MDBRow,
} from "mdb-react-ui-kit";

interface Props {
  user: string;
  onChangeOrigin: (genre: string) => void;
  onChangePage: (page: number) => void;
}

//这层保存所有分叉设置
function SelectWorkGenre(props: Props) {
  const [button1Text, setButton1Text] = React.useState("选择"); //外文翻译选择
  const [button1State, setButton1State] = React.useState("btn btn-success");
  const [button2Text, setButton2Text] = React.useState("选择"); //原创文章选择
  const [button2State, setButton2State] = React.useState("btn btn-success");
  const [genre, setGenre] = React.useState(""); //选择的分区
  const [owner, setUser] = React.useState(props.user);
  const [repo, setrepo] = React.useState("");

  const handleCreateRepo = (owner: string, repo: string) => {
    setUser(owner);
    setrepo(repo);
  };

  const handlebutton1Click = () => {
    if (button1Text === "选择") {
      setButton1Text("已选择！");
      setButton2Text("选择");
      setButton1State("disabled btn btn-success");
      setButton2State("btn btn-success");
    }
  };
  const handlebutton2Click = () => {
    if (button2Text === "选择") {
      setButton2Text("已选择！");
      setButton1Text("选择");
      setButton2State("disabled btn btn-success");
      setButton1State("btn btn-success");
    }
  };

  const completeSelection = (genre: string, isOldFork: number) => {
    if (isOldFork === 0) {
      alert("未选择分叉");
      return;
    }
    if (owner === "") {
      alert("未选择用户");
      return;
    }
    if (isOldFork === 1) {
      setGenre(genre);
      props.onChangePage(1);
      props.onChangeOrigin(genre);
      console.log("#####保存以下信息：");
      console.log("用户：" + props.user); //这里是github 用户名，之后配合后端改
      console.log("使用现有分叉");
      console.log("分区：" + genre);
    } else {
      if (repo === "") {
        alert("未命名库");
        return;
      }

      setGenre(genre);
      props.onChangePage(1);
      props.onChangeOrigin(genre);
      console.log("#####保存以下信息：");
      console.log("新建分叉");
      console.log("owner：" + owner);
      console.log("库名：" + repo);
      console.log("分区：" + genre);
    }
  };

  return (
    <>
      <MDBContainer
        fluid
        className="d-flex justify-content-center align-items-cente"
        style={{ marginTop: "20px" }}
      >
        <MDBCol size="10" className="d-flex ">
          <h3>选择贡献分区</h3>
        </MDBCol>
      </MDBContainer>
      <MDBContainer
        fluid
        className="d-flex justify-content-center align-items-cente"
        style={{ marginTop: "20px" }}
      >
        <MDBCol size="10" className="d-flex mx-2">
          <MDBRow>
            <MDBCol sm="5">
              <MDBCard>
                <MDBCardBody>
                  <MDBCardTitle>外文文章翻译</MDBCardTitle>
                  <MDBCardText>
                    FOSScope/TranslateProject - 开源观察翻译项目
                  </MDBCardText>
                  <MDBBtn className={button1State} onClick={handlebutton1Click}>
                    {button1Text}
                  </MDBBtn>
                </MDBCardBody>
              </MDBCard>
            </MDBCol>
            <MDBCol sm="5">
              <MDBCard>
                <MDBCardBody>
                  <MDBCardTitle>原创中文文章&中文文章转载</MDBCardTitle>
                  <MDBCardText>
                    FOSScope/Articles - 开源观察原创文章与中文转载文章源文件
                  </MDBCardText>
                  <MDBBtn className={button2State} onClick={handlebutton2Click}>
                    {button2Text}
                  </MDBBtn>
                </MDBCardBody>
              </MDBCard>
            </MDBCol>
          </MDBRow>
        </MDBCol>
      </MDBContainer>
      {button1Text === "选择" && button2Text === "选择" ? (
        <></>
      ) : (
        <>
          {button1Text === "已选择！" ? (
            <TransForkRepo
              saveGenre={completeSelection}
              repo={repo}
              saveRepo={handleCreateRepo}
              user={props.user}
            ></TransForkRepo>
          ) : (
            <OrigForkRepo
              saveGenre={completeSelection}
              repo={repo}
              saveRepo={handleCreateRepo}
              user={props.user}
            ></OrigForkRepo>
          )}
        </>
      )}
    </>
  );
}

export default SelectWorkGenre;
