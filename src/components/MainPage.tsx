import React, { useState } from "react";
import SelectWorkGenre from "./SelectWorkGenre";
import {
  MDBNavbar,
  MDBContainer,
  MDBIcon,
  MDBNavbarNav,
  MDBNavbarItem,
  MDBNavbarLink,
  MDBNavbarToggler,
  MDBNavbarBrand,
  MDBCollapse,
  MDBBtn,
  MDBCol,
} from "mdb-react-ui-kit";
import TranslationProcedure from "./TranslationProcedure/TranslationProcedure";
import OriginalWorkProcedure from "./OriginalWorkProcedure/OriginalWorkProcedure";

interface Props {
  onChangPage: (page: boolean) => void;
  user: string;
  token: string;
  deepseek: string;
}

export default function App(props: Props) {
  //css
  const [openNavColor, setOpenNavColor] = useState(false);

  //第0页为选择分区，后续是各分区后续页码
  const [page, setPage] = useState(0);

  //0为未选择，1为选择原创，2为选择翻译
  const [isOrigin, setIsOrigin] = useState(0);

  const onChangeOrigin = (genre: string) => {
    setIsOrigin(genre === "原创" ? 1 : 2);
  };

  const handlePageChange = (page: number) => {
    setPage(page);
  };

  return (
    <>
      <MDBNavbar expand="lg" dark bgColor="success">
        <MDBContainer fluid>
          <MDBNavbarBrand href="#">FOSScope Toolkit</MDBNavbarBrand>
          <MDBNavbarToggler
            type="button"
            data-target="#navbarColor02"
            aria-controls="navbarColor02"
            aria-expanded="false"
            aria-label="Toggle navigation"
            onClick={() => setOpenNavColor(!openNavColor)}
          >
            <MDBIcon icon="bars" fas />
          </MDBNavbarToggler>
          <MDBCollapse open={openNavColor} navbar>
            <MDBNavbarNav className="me-auto mb-2 mb-lg-0">
              <MDBNavbarItem>
                <MDBNavbarLink href="">使用指南</MDBNavbarLink>
                {/* 完全没写，不需要就删掉 */}
              </MDBNavbarItem>
            </MDBNavbarNav>
          </MDBCollapse>
        </MDBContainer>
      </MDBNavbar>

      <MDBContainer
        fluid
        className="d-flex justify-content-center align-items-center"
        style={{ marginTop: "20px" }}
      >
        <MDBCol
          size="10"
          className="d-flex justify-content-between align-items-center"
        >
          {/* 顶部那仨 */}
          <div style={{ flex: "1" }}>
            <p className="text-start">Github 用户名：{props.user}</p>
            <p>Github Token：{props.token}</p>
            <p>DeepSeek API Key：{props.deepseek}</p>
          </div>

          <MDBBtn
            className="btn btn-success"
            onClick={() => props.onChangPage(false)}
          >
            更改
          </MDBBtn>
        </MDBCol>
      </MDBContainer>

      <MDBContainer
        fluid
        className="d-flex justify-content-center align-items-center"
        style={{ marginTop: "20px" }}
      >
        <MDBCol size="10" className="d-flex ">
          {isOrigin === 0 && (
            <>
              <MDBBtn outline color="success">
                <MDBIcon fas icon="lock-open" /> 1. 选择分区
              </MDBBtn>
              <MDBBtn outline className="mx-2" color="success">
                <MDBIcon fas icon="lock" />
              </MDBBtn>
            </>
          )}
          {/* 展示对应的选项卡组 */}
          {isOrigin === 1 && (
            <>
              <MDBBtn outline color="success" onClick={() => setPage(0)}>
                <MDBIcon fas icon="lock-open" /> 1. 选择分区
              </MDBBtn>
              <MDBBtn
                outline
                className="mx-2"
                color="success"
                onClick={() => setPage(1)}
              >
                <MDBIcon fas icon="lock-open" />
                2. 编辑文章
              </MDBBtn>
              <MDBBtn
                outline
                className="mx-2"
                color="success"
                onClick={() => setPage(2)}
              >
                <MDBIcon fas icon="lock-open" />
                3. 校对预览
              </MDBBtn>
              <MDBBtn
                outline
                className="mx-2"
                color="success"
                onClick={() => setPage(3)}
              >
                <MDBIcon fas icon="lock-open" />
                4. 发布
              </MDBBtn>
            </>
          )}
          {isOrigin === 2 && (
            <>
              <MDBBtn outline color="success" onClick={() => setPage(0)}>
                <MDBIcon fas icon="lock-open" /> 1. 选择分区
              </MDBBtn>
              <MDBBtn
                outline
                className="mx-2"
                color="success"
                onClick={() => setPage(1)}
              >
                <MDBIcon fas icon="lock-open" />
                2. 选题
              </MDBBtn>
              <MDBBtn
                outline
                className="mx-2"
                color="success"
                onClick={() => setPage(2)}
              >
                <MDBIcon fas icon="lock-open" />
                3. 翻译
              </MDBBtn>
              <MDBBtn
                outline
                className="mx-2"
                color="success"
                onClick={() => setPage(3)}
              >
                <MDBIcon fas icon="lock-open" />
                4. 校对
              </MDBBtn>
              <MDBBtn
                outline
                className="mx-2"
                color="success"
                onClick={() => setPage(4)}
              >
                <MDBIcon fas icon="lock-open" />
                5. 发布
              </MDBBtn>
            </>
          )}
        </MDBCol>
      </MDBContainer>
      {/* 展示选项卡对应内容 */}
      {page === 0 && (
        <SelectWorkGenre
          user={props.user}
          onChangeOrigin={onChangeOrigin}
          onChangePage={handlePageChange}
        />
      )}
      {page != 0 && isOrigin === 1 && <OriginalWorkProcedure page={page} />}
      {page != 0 && isOrigin === 2 && <TranslationProcedure page={page} />}
    </>
  );
}
