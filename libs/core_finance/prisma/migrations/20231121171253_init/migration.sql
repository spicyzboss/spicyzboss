-- CreateTable
CREATE TABLE "transactions" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "transaction_type_id" INTEGER NOT NULL,
    "amount" INTEGER NOT NULL,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "transactions_transaction_type_id_fkey" FOREIGN KEY ("transaction_type_id") REFERENCES "transaction_types" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "transaction_types" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- CreateTable
CREATE TABLE "tags" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- CreateTable
CREATE TABLE "transactions_tags" (
    "transaction_id" TEXT NOT NULL,
    "tag_id" INTEGER NOT NULL,
    "created_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY ("transaction_id", "tag_id"),
    CONSTRAINT "transactions_tags_transaction_id_fkey" FOREIGN KEY ("transaction_id") REFERENCES "transactions" ("id") ON DELETE RESTRICT ON UPDATE CASCADE,
    CONSTRAINT "transactions_tags_tag_id_fkey" FOREIGN KEY ("tag_id") REFERENCES "tags" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);

-- CreateIndex
CREATE UNIQUE INDEX "tags_name_key" ON "tags"("name");
