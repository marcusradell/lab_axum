-- CreateTable
CREATE TABLE "IdentityEvent" (
    "sequence_num" BIGSERIAL NOT NULL,
    "stream_id" TEXT NOT NULL,
    "version" INTEGER NOT NULL,
    "event_type" TEXT NOT NULL,
    "data" JSONB NOT NULL,
    "cid" TEXT NOT NULL,
    "inserted_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "IdentityEvent_pkey" PRIMARY KEY ("sequence_num")
);

-- CreateIndex
CREATE INDEX "IdentityEvent_stream_id_version_idx" ON "IdentityEvent"("stream_id", "version");

-- CreateIndex
CREATE UNIQUE INDEX "IdentityEvent_stream_id_version_key" ON "IdentityEvent"("stream_id", "version");
