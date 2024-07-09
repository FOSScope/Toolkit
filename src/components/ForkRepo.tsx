import React from "react";
import {
  MDBAccordion,
  MDBAccordionItem,
  MDBBtn,
  MDBInputGroup,
} from "mdb-react-ui-kit";

interface props {
  onEdit: () => void;
  user: string;
  repo: string;
  saveRepo: (user: string, repo: string) => void;
  isEdit: boolean;
}

function ForkRepo(props: props) {
  const [localUser, setLocalUser] = React.useState(props.user);
  const [repoName, setRepoName] = React.useState(props.repo);

  const handleOwnerChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setLocalUser(event.target.value);
  };

  const handleRepoChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setRepoName(event.target.value);
  };

  const handleSave = () => {
    props.saveRepo(localUser, repoName);
    props.onEdit();
  };

  return (
    <MDBAccordion initialActive={1} className="w-100">
      <MDBAccordionItem
        collapseId={1}
        headerTitle={<span className="text-success">创建分叉库</span>}
      >
        <h6 className="text-secondary">Owner / Repository</h6>
        {props.isEdit ? (
          <>
            <MDBInputGroup className="mb-3">
              <input
                className="form-control"
                value={localUser}
                onChange={handleOwnerChange}
                type="text"
              />
              <span className="input-group-text">/</span>
              <input
                className="form-control"
                placeholder="Repository name"
                value={repoName}
                onChange={handleRepoChange}
                type="text"
              />
            </MDBInputGroup>
            <MDBBtn rounded color="success" onClick={handleSave}>
              保存
            </MDBBtn>{" "}
          </>
        ) : (
          <>
            <p>
              {localUser} / {repoName}
            </p>
            <MDBBtn rounded color="success" onClick={props.onEdit}>
              更改
            </MDBBtn>
          </>
        )}
      </MDBAccordionItem>
    </MDBAccordion>
  );
}

export default ForkRepo;
