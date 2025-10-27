import icechunk
import tempfile
import zarr
from zarr.dtype import VariableLengthUTF8

location = tempfile.TemporaryDirectory().name 
storage = icechunk.local_filesystem_storage(location)
repo = icechunk.Repository.create(storage)
session = repo.writable_session("main")

root = zarr.open_group(session.store, mode="w", zarr_format=3)
meta = root.create_group("meta")

array = meta.create_array(
    "collection",
    shape=(3,),
    dtype=VariableLengthUTF8(),
)
meta["collection"][...] = ["collection_a"]

session.commit("First")
print(location)
