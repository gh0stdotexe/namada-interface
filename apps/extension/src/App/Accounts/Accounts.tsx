import { useNavigate } from "react-router-dom";
import { Icon, IconName } from "@anoma/components";

import { TopLevelRoute } from "App/types";
import { DerivedAccount } from "background/keyring";
import {
  AccountsContainer,
  AccountsList,
  AccountsListItem,
  ButtonContainer,
  Button,
  ButtonText,
} from "./Accounts.components";
import { AccountListing } from "App/Accounts";

type Props = {
  accounts: DerivedAccount[];
};

const Accounts = ({ accounts }: Props): JSX.Element => {
  const navigate = useNavigate();

  return (
    <AccountsContainer>
      <AccountsList>
        {accounts.map((account) => (
          <AccountsListItem key={`account-${account.id}`}>
            <AccountListing account={account} />
          </AccountsListItem>
        ))}
      </AccountsList>
      <ButtonContainer>
        <Button onClick={() => navigate(TopLevelRoute.WalletAddAccount)}>
          <ButtonText>Derive new account</ButtonText>
          <Icon iconName={IconName.Plus} />
        </Button>
      </ButtonContainer>
    </AccountsContainer>
  );
};

export default Accounts;