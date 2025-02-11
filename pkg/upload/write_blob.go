package upload

import (
	"context"
	"encoding/hex"
	"io"
	"time"

	"github.com/treeverse/lakefs/pkg/block"
)

type Blob struct {
	PhysicalAddress string
	RelativePath    bool
	Checksum        string
	Size            int64
	CreationDate    time.Time
}

func WriteBlob(ctx context.Context, adapter block.Adapter, objectPointer block.ObjectPointer, body io.Reader, contentLength int64, opts block.PutOpts) (*Blob, error) {
	// handle the upload itself
	hashReader := block.NewHashingReader(body, block.HashFunctionMD5, block.HashFunctionSHA256)
	res, err := adapter.Put(ctx, objectPointer, contentLength, hashReader, opts)
	if err != nil {
		return nil, err
	}
	checksum := hex.EncodeToString(hashReader.Md5.Sum(nil))
	return &Blob{
		PhysicalAddress: objectPointer.Identifier,
		RelativePath:    true,
		Checksum:        checksum,
		Size:            hashReader.CopiedSize,
		CreationDate:    res.GetMtime(),
	}, nil
}
