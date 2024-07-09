import React from "react";
import EditWork from "./EditWork";
import ProofreadingO from "./ProofreadingO";
import PublishO from "./PublishO";

interface props {
  page: Number;
}

function OriginalWorkProcedure(props: props) {
  switch (props.page) {
    case 1:
      return <EditWork />;
    case 2:
      return <ProofreadingO />;
    case 3:
      return <PublishO />;
  }
}

export default OriginalWorkProcedure;
