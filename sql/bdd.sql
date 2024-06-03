CREATE TABLE pks (
    userId          BIGINT,
    pk1             VARCHAR(255),
    pk2             VARCHAR(255),
    pk3             VARCHAR(255)
);

CREATE TABLE basics (
    userId          BIGINT,
    chain           VARCHAR(50)
);

CREATE TABLE login (
    userId      INTEGER,
    pass        VARCHAR(255)
);

CREATE TABLE referred (
    userId          BIGINT,
    refaddress      VARCHAR(255)
);

CREATE TABLE referrals (
    username    VARCHAR(255),
    refaddress  VARCHAR(255)
);

CREATE TABLE tokens (
    tokenaddress   VARCHAR(255),
    symbol      VARCHAR(255),
    decimals    INTEGER,
    chainid     INTEGER
);

CREATE TABLE buytokens (
    userid          INTEGER,
    privatetx       BOOLEAN DEFAULT true,
    failguard       BOOLEAN DEFAULT false,
    frontrun        BOOLEAN DEFAULT false,
    ercaddress      VARCHAR(255),
    lastmenuid      INTEGER DEFAULT 0,
    wallet          INTEGER DEFAULT 1,
    selectamount    FLOAT DEFAULT 1,
    selectslippage  FLOAT DEFAULT 10,
    selltax         INTEGER DEFAULT 0,
    buytax          INTEGER DEFAULT 0,
    gasestimation   VARCHAR(255) DEFAULT 0
);

CREATE TABLE selltokens (
    userid          INTEGER,
    privatetx       BOOLEAN DEFAULT true,
    failguard       BOOLEAN DEFAULT false,
    frontrun        BOOLEAN DEFAULT false,
    lastmenuid      INTEGER DEFAULT 0,
    wallet          INTEGER DEFAULT 1,
    selectamount    FLOAT DEFAULT 1,
    selectslippage  FLOAT DEFAULT 10,
    gasestimation   VARCHAR(255) DEFAULT 0
);

CREATE TABLE buylimit (
    userid          INTEGER,
    lastmenuid      INTEGER DEFAULT 0,
    wallet          INTEGER DEFAULT 1,
    selectamount    FLOAT DEFAULT 1,
    ercaddress      VARCHAR(255) DEFAULT '',
    limitorder      FLOAT DEFAULT 10,
    expirationhour  INTEGER DEFAULT 24
);

CREATE TABLE buyorders (
    id              INTEGER,
    userid          INTEGER,
    wallet          INTEGER,
    symbol          VARCHAR(255),
    makerAsset      VARCHAR(255),
    takerAsset      VARCHAR(255),
    amountIn        VARCHAR(255),
    amountOut       VARCHAR(255),
    outDecimals     INTEGER,
    timed           TIMESTAMP,
    salt            VARCHAR(255),
    expiration      VARCHAR(255)
);


CREATE TABLE selllimit (
    userid          INTEGER,
    lastmenuid      INTEGER DEFAULT 0,
    wallet          INTEGER DEFAULT 1,
    selectamount    FLOAT DEFAULT 1,
    ercaddress      VARCHAR(255),
    limitorder      FLOAT DEFAULT 10,
    expirationhour  INTEGER DEFAULT 24
);

CREATE TABLE sellorders (
    id              INTEGER,
    userid          INTEGER,
    wallet          INTEGER,
    symbol          VARCHAR(255),
    makerAsset      VARCHAR(255),
    takerAsset      VARCHAR(255),
    amountIn        VARCHAR(255),
    amountOut       VARCHAR(255),
    outDecimals     INTEGER,
    timed           TIMESTAMP,
    salt            VARCHAR(255),
    expiration      VARCHAR(255)
);

CREATE TABLE transfer (
    userid          INTEGER,
    fromwallet      INTEGER DEFAULT 0,
    towallet        VARCHAR(255),
    lastmenuid      INTEGER DEFAULT 0,
    customtoken     VARCHAR(42) DEFAULT 'ETH',
    selectamount    FLOAT DEFAULT 1
);

CREATE TABLE bridge (
    userid          INTEGER,
    fromwallet      INTEGER DEFAULT 1,
    towallet        VARCHAR(42),
    lastmenuid      INTEGER DEFAULT 0,
    selectamount    FLOAT DEFAULT 1,
    selecttoken     INTEGER,
    lasttokensbalances VARCHAR(255) DEFAULT '',
    selectnetwork       VARCHAR(255) DEFAULT ''
);

CREATE TABLE registered (
    userId BIGINT,
    firstMsgId BIGINT
);


