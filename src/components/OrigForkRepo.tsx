import React from "react";
import {
  MDBContainer,
  MDBCol,
  MDBCard,
  MDBCardBody,
  MDBCardText,
  MDBBtn,
  MDBRow,
  MDBAccordion,
  MDBAccordionItem,
} from "mdb-react-ui-kit";
import ForkRepo from "./ForkRepo";

interface props {
  repo: string;
  user: string;
  saveRepo: (user: string, repo: string) => void;
  saveGenre: (genre: string, isOldFork: number) => void;
}

function OrigForkRepo(props: props) {
  const [button1Text, setButton1Text] = React.useState("选择"); //使用现有
  const [button1State, setButton1State] = React.useState("btn btn-success");
  const [button2Text, setButton2Text] = React.useState("选择"); //创建新的
  const [button2State, setButton2State] = React.useState("btn btn-success");
  const [isOldFork, setIsOldFork] = React.useState(0); //0表示未选择，1表示选择了使用现有分叉库，2表示选择了创建新分叉库

  const [isEdit, setisEdit] = React.useState(true);

  const onEdit = () => {
    setisEdit(!isEdit);
  };

  const handlebutton1Click = () => {
    if (button1Text === "选择") {
      setButton1Text("已选择！");
      setButton2Text("选择");
      setButton1State("disabled btn btn-success");
      setButton2State("btn btn-success");
      setIsOldFork(1);
    }
  };
  const handlebutton2Click = () => {
    if (button2Text === "选择") {
      setButton2Text("已选择！");
      setButton1Text("选择");
      setButton2State("disabled btn btn-success");
      setButton1State("btn btn-success");
      setIsOldFork(2);
    }
  };

  const handleCompleteButtonClick = () => {
    props.saveGenre("原创", isOldFork);
  };

  return (
    <>
      <MDBContainer
        fluid
        className="d-flex justify-content-center align-items-cente"
        style={{ marginTop: "20px" }}
      >
        <MDBCol size="10" className="d-flex mx-2">
          <MDBRow className="w-100">
            <MDBCol sm="3">
              <MDBCard>
                <MDBCardBody>
                  <MDBCardText>使用现有分叉库</MDBCardText>
                  <MDBBtn className={button1State} onClick={handlebutton1Click}>
                    {button1Text}
                  </MDBBtn>
                </MDBCardBody>
              </MDBCard>
            </MDBCol>
            <MDBCol sm="3">
              <MDBCard>
                <MDBCardBody>
                  <MDBCardText>创建新分叉库</MDBCardText>
                  <MDBBtn className={button2State} onClick={handlebutton2Click}>
                    {button2Text}
                  </MDBBtn>
                </MDBCardBody>
              </MDBCard>
            </MDBCol>
          </MDBRow>
        </MDBCol>
      </MDBContainer>
      <MDBContainer
        fluid
        className="d-flex justify-content-center align-items-cente"
        style={{ marginTop: "20px" }}
      >
        <MDBCol size="10" className="d-flex mx-2">
          {isOldFork === 1 && (
            <MDBAccordion initialActive={1} className="w-100">
              <MDBAccordionItem
                collapseId={1}
                headerTitle={<span className="text-success">现有分叉库</span>}
              >
                <p>owner/name：abcd（未完成看后端）/（也未完成看后端）</p>
                <MDBBtn rounded color="success">
                  更换
                </MDBBtn>
              </MDBAccordionItem>
            </MDBAccordion>
          )}
          {isOldFork === 2 && (
            <ForkRepo
              repo={props.repo}
              saveRepo={props.saveRepo}
              onEdit={onEdit}
              user={props.user}
              isEdit={isEdit}
            ></ForkRepo>
          )}
        </MDBCol>
      </MDBContainer>
      <MDBContainer
        fluid
        className="d-flex justify-content-center align-items-cente"
        style={{ marginTop: "20px", marginBottom: "10vh" }}
      >
        <MDBCol size="10" className="d-flex justify-content-end">
          <MDBBtn
            className="btn btn-success"
            onClick={handleCompleteButtonClick}
          >
            保存
          </MDBBtn>
        </MDBCol>
      </MDBContainer>
    </>
  );
}

export default OrigForkRepo;
