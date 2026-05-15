-- CreateEnum
CREATE TYPE "OrderStatus" AS ENUM ('NEW', 'OPEN', 'FILLED', 'CANCELED');

-- CreateEnum
CREATE TYPE "OrderType" AS ENUM ('LIMIT', 'MARKET');

-- CreateEnum
CREATE TYPE "Side" AS ENUM ('BUY', 'SELL');

-- CreateEnum
CREATE TYPE "MarketStatus" AS ENUM ('OPEN', 'PAUSED', 'CLOSED');

-- CreateTable
CREATE TABLE "Market" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "status" "MarketStatus" NOT NULL DEFAULT 'OPEN',
    "baseId" TEXT NOT NULL,
    "quoteId" TEXT NOT NULL,
    "tickSize" DECIMAL(38,18) NOT NULL,
    "lotSize" DECIMAL(38,18) NOT NULL,
    "minQty" DECIMAL(38,18) NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Market_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Asset" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "symbol" TEXT NOT NULL,
    "decimals" INTEGER NOT NULL,

    CONSTRAINT "Asset_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Balance" (
    "id" TEXT NOT NULL,
    "userId" TEXT NOT NULL,
    "locked" DECIMAL(38,18) NOT NULL,
    "available" DECIMAL(38,18) NOT NULL,
    "assetId" TEXT NOT NULL,
    "updatedAt" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "Balance_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Trade" (
    "id" TEXT NOT NULL,
    "marketId" TEXT,

    CONSTRAINT "Trade_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Order" (
    "id" TEXT NOT NULL,
    "clientOrderId" TEXT NOT NULL,
    "userId" TEXT NOT NULL,
    "marketId" TEXT NOT NULL,
    "side" "Side" NOT NULL,
    "type" "OrderType" NOT NULL,
    "price" DECIMAL(38,18) NOT NULL,
    "quantity" DECIMAL(38,18) NOT NULL,
    "filledQuantity" DECIMAL(38,18) NOT NULL DEFAULT 0,
    "status" "OrderStatus" NOT NULL DEFAULT 'NEW',
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "Order_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE INDEX "Order_userId_status_idx" ON "Order"("userId", "status");

-- CreateIndex
CREATE INDEX "Order_marketId_status_idx" ON "Order"("marketId", "status");

-- CreateIndex
CREATE UNIQUE INDEX "Order_userId_clientOrderId_key" ON "Order"("userId", "clientOrderId");

-- AddForeignKey
ALTER TABLE "Market" ADD CONSTRAINT "Market_baseId_fkey" FOREIGN KEY ("baseId") REFERENCES "Asset"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Market" ADD CONSTRAINT "Market_quoteId_fkey" FOREIGN KEY ("quoteId") REFERENCES "Asset"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Balance" ADD CONSTRAINT "Balance_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Balance" ADD CONSTRAINT "Balance_assetId_fkey" FOREIGN KEY ("assetId") REFERENCES "Asset"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Trade" ADD CONSTRAINT "Trade_marketId_fkey" FOREIGN KEY ("marketId") REFERENCES "Market"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Order" ADD CONSTRAINT "Order_userId_fkey" FOREIGN KEY ("userId") REFERENCES "User"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "Order" ADD CONSTRAINT "Order_marketId_fkey" FOREIGN KEY ("marketId") REFERENCES "Market"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
