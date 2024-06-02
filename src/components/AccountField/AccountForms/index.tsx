import { ObservableObject, ObservablePrimitiveChildFns } from '@legendapp/state'
import { Box, Button, Flex, Select, Tabs, Text, TextField } from '@radix-ui/themes'
import { IAccount, IDomain } from '../../..'
import { ResStatusHelpers, TaskHelpers } from '../../../core/util'
import { AccountReqType, accountTaskHelper } from '../../../core/state/account'
import { observer } from '@legendapp/state/react'


type Props = {
  domains: IDomain[]
  addAccount: () => Promise<void>
  selectedDomain: ObservableObject<string>
  accountTaskHelper: ReturnType<typeof TaskHelpers<AccountReqType>>
  stateResStatusHelper: ReturnType<typeof ResStatusHelpers<AccountReqType>>
  input: ObservableObject<Partial<IAccount>>
  addType: ObservablePrimitiveChildFns<string>
}

export const AccountForms = observer((p: Props) => {
  const isCreateReq = !!accountTaskHelper.findTaskByReqType('account', 'create')
  return (
    <Flex direction="column" gap="3" width="260px">
      <Tabs.Root defaultValue="email" onValueChange={(e: string) => p.addType.set(e)}>
        <Tabs.List>
          <Tabs.Trigger value="email">Email</Tabs.Trigger>
          <Tabs.Trigger value="domain">Domain</Tabs.Trigger>
        </Tabs.List>

        <Box pt="3">
          <Tabs.Content value="email">
            <EmailForm
              input={p.input}
              isCreateReq={isCreateReq}
              // stateResStatusHelper={p.stateResStatusHelper}
              // accountTaskHelper={p.accountTaskHelper}
            />
          </Tabs.Content>

          <Tabs.Content value="domain">
            <DomainForm
              domains={p.domains}
              selectedDomain={p.selectedDomain}
              isCreateReq={isCreateReq}
            />
          </Tabs.Content>
        </Box>
      </Tabs.Root>
      <Box width="100px" mb="3">
        <Button size="1" disabled={isCreateReq} onClick={() => p.addAccount()}>
          Add Account
        </Button>
      </Box>
    </Flex>
  )
})

type EmailProps = {
  input: ObservableObject<Partial<IAccount>>
  // addAccount: (e: FormEvent<HTMLFormElement>) => Promise<void>
  // accountTaskHelper: ReturnType<typeof TaskHelpers<AccountReqType>>
  // stateResStatusHelper: ReturnType<typeof ResStatusHelpers<AccountReqType>>
  isCreateReq: boolean
}

export const EmailForm = observer((p: EmailProps) => {
  return (
    <Flex direction="column" gap="1">
      <Flex align="center" gap="3">
        <label>
          <Text as="div" size="2" className="mr-[25px]">
            Email:
          </Text>
        </label>
        <TextField.Root
          disabled={p.isCreateReq}
          className="w-[12rem]"
          size="1"
          placeholder="Enter your email"
          value={p.input.email.get()}
          onChange={(e) => {
            p.input.set((p) => ({ ...p, email: e.target.value }))
          }}
        />
      </Flex>

      <Flex align="center" gap="3">
        <label>
          <Text as="div" size="2">
            Password:
          </Text>
        </label>
        <TextField.Root
          disabled={p.isCreateReq}
          className="w-[12rem]"
          size="1"
          placeholder="Enter your password"
          value={p.input.password.get()}
          onChange={(e) => {
            p.input.set((p) => ({ ...p, password: e.target.value }))
          }}
        />
      </Flex>
    </Flex>
  )
})

type DomainProps = {
  domains: IDomain[]
  // addAccount: (e: FormEvent<HTMLFormElement>) => Promise<void>
  selectedDomain: ObservableObject<string>
  // accountTaskHelper: ReturnType<typeof TaskHelpers<AccountReqType>>
  // stateResStatusHelper: ReturnType<typeof ResStatusHelpers<AccountReqType>>
  isCreateReq: boolean
}

export const DomainForm = observer((p: DomainProps) => {
  return (
    <Flex direction="column" gap="1">
      <Select.Root disabled={p.isCreateReq} onValueChange={p.selectedDomain.set}>
        <Select.Trigger placeholder="Select a domain" />
        <Select.Content>
          <Select.Group>
            <Select.Label>Verified</Select.Label>
            {p.domains
              .filter((d) => d.verified)
              .map((d, idx) => (
                  <Select.Item key={idx} value={d.domain}>
                    {d.domain}
                  </Select.Item>
                )
              )}
          </Select.Group>

          <Select.Separator />

          <Select.Group>
            <Select.Label>Unverified</Select.Label>
            {p.domains
              .filter((d) => !d.verified)
              .map((d, idx) => {
                return (
                  <Select.Item key={idx} value={d.domain} disabled>
                    {d.domain}
                  </Select.Item>
                )
              })}
          </Select.Group>
        </Select.Content>
      </Select.Root>
    </Flex>
  )
})
