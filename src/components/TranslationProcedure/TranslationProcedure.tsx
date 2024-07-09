import React from "react";
import ChooseTitle from "./ChooseTitle";
import Translation from "./Translation";
import ProofreadingT from "./ProofreadingT";
import PublishT from "./PublishT";

interface props {
  page: Number;
}

function TranslationProcedure(props: props) {
  switch (props.page) {
    case 1:
      return <ChooseTitle />;

    case 2:
      return <Translation />;

    case 3:
      return <ProofreadingT />;

    case 4:
      return <PublishT />;
  }
}

export default TranslationProcedure;
