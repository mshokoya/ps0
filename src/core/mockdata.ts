import { IAccount, IDomain, IMetaData, IRecords } from ".."

export const domainMockData: IDomain[] = [
  {
    id: 'dasdsaas1',
    domain: 'dasdsaas12',
    verified: true,
    mx_records: true,
    txt_records: true,
    message: "dsafsas"
  },
  {
    id: 'dasdsaas2',
    domain: 'dasdsaas23',
    verified: false,
    mx_records: true,
    txt_records: false,
    message: "dsafsas"
  },
  {
    id: 'dasdsaas3',
    domain: 'dasdsaas34',
    verified: true,
    mx_records: true,
    txt_records: true,
    message: "dsafsas"
  },
  {
    id: 'dasdsaas4',
    domain: 'dasdsaas4321',
    verified: false,
    mx_records: false,
    txt_records: true,
    message: "dsafsas"
  },
  {
    id: 'dasdsaas5',
    domain: 'dasdsaas234',
    verified: true,
    mx_records: true,
    txt_records: true,
    message: "dsafsas"
  },
  {
    id: 'dasdsaas6',
    domain: 'das423412dsaas',
    verified: true,
    mx_records: true,
    txt_records: true,
    message: "dsafsas"
  },
] 

export const accountMockData: IAccount[] = [
  {
    id: '123451',
    domain: 'domain',
    suspended: false,
    email: 'msefdsadsadefsdfsdfssadswdaedasdsasada@h.co.uk',
    password: 'pass',
    cookies: 'dasdasdas',
    proxy: 'dsaasd',
    last_used: new Date().getTime(),
    verified: "no",
    login_type: "default",
    credits_limit: 100,
    credits_used: 300,
    renewal_date: 2121, 
    renewal_start_date: 532, 
    renewal_end_date: 54234, 
    trial_days_left: 3121, 
    history: [
      {list_name: "dasass", scrape_id: "dsadsadsas", scrape_time: 23432, total_page_scrape: 54}
    ]
  },
  {
    id: '123452',
    domain: 'domain',
    suspended: false,
    email: 'msefdsadsadefsdfsdfssadswdaedasdsasada@h.co.uk',
    password: 'pass',
    cookies: 'dasdasdas',
    proxy: 'dsaasd',
    last_used: new Date().getTime(),
    verified: "no",
    login_type: "default",
    credits_limit: 100,
    credits_used: 300,
    renewal_date: 2121, 
    renewal_start_date: 532, 
    renewal_end_date: 54234, 
    trial_days_left: 3121, 
    history: [
      {list_name: "dasass", scrape_id: "dsadsadsas", scrape_time: 23432, total_page_scrape: 54}
    ]
  },
  {
    id: '123453',
    domain: 'domain',
    suspended: false,
    email: 'msefdsadsadefsdfsdfssadswdaedasdsasada@h.co.uk',
    password: 'pass',
    cookies: 'dasdasdas',
    proxy: 'dsaasd',
    last_used: new Date().getTime(),
    verified: "no",
    login_type: "default",
    credits_limit: 100,
    credits_used: 300,
    renewal_date: 2121, 
    renewal_start_date: 532, 
    renewal_end_date: 54234, 
    trial_days_left: 3121, 
    history: [
      {list_name: "dasass", scrape_id: "dsadsadsas", scrape_time: 23432, total_page_scrape: 54}
    ]
  },
  {
    id: '123454',
    domain: 'domain',
    suspended: false,
    email: 'msefdsadsadefsdfsdfssadswdaedasdsasada@h.co.uk',
    password: 'pass',
    cookies: 'dasdasdas',
    proxy: 'dsaasd',
    last_used: new Date().getTime(),
    verified: "no",
    login_type: "default",
    credits_limit: 100,
    credits_used: 300,
    renewal_date: 2121, 
    renewal_start_date: 532, 
    renewal_end_date: 54234, 
    trial_days_left: 3121, 
    history: [
      {list_name: "dasass", scrape_id: "dsadsadsas", scrape_time: 23432, total_page_scrape: 54}
    ]
  },
]

export const metaMockData: IMetaData[] = [
  {
    id: '123451',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
  {
    id: '123452',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
  {
    id: '123453',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
  {
    id: '123454',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
  {
    id: '123455',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
  {
    id: '123456',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
  {
    id: '123457',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
  {
    id: '123458',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
  {
    id: '123459',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
  {
    id: '123450',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
  {
    id: '1234510',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
  {
    id: '1234511',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
  {
    id: '1234512',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
  {
    id: '1234513',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
  {
    id: '1234514',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
  {
    id: '1234515',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
  {
    id: '1234517',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
  {
    id: '1234516',
    url: 'www.gom',
    params: { lol: 'fds', poll: 'cascas' },
    name: 'dad',
    scrapes: [
      { scrape_id: "dwadsaads", list_name: "dwdwq", length: 23, date: 1826 },
      { scrape_id: "dwadsdasdsads", list_name: "dsadsa", length: 32, date: 29376 }
    ],
    accounts: [
      { account_id: 'dsadsawe',range: [1, 200] },
      { account_id: 'dascew', range: [201, 500] },
      { account_id: 'erfdswer', range: [501, 1000] }
    ]
  },
]

export const recordMockData: IRecords[] = [
  {
    id: '123451',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '123452',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '123453',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '123454',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '123455',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '123456',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '123457',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '123458',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '123459',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '123450',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234510',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234511',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234512',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234513',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234514',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234515',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234516',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234517',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234518',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234519',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234520',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234521',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234522',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234523',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234524',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234525',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234526',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234527',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234528',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234529',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234530',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
  {
    id: '1234531',
    scrape_id: 'd',
    url: 'http:/dsadsa.com',
    data: {
      name: 'mike',
      linkedin: 'fsd',
      title: 'fdsfds',
      firstname: 'dsadsaas',
      lastname: 'dssadsads',
      company_name: 'fdsjhbknlmjhjgvhjbknhjghjkbnhbjgkw',
      company_website: 'dsfw',
      company_linkedin: 'thre',
      company_twitter: 'grev',
      company_facebook: 'tgerf',
      email: 'dfsfew',
      is_verified: true,
      company_location: 'cvfgd',
      employees: '8',
      phone: '324532',
      industry: 'fsdgre',
      keywords: ['fdssf', 'fdse']
    }
  },
]
