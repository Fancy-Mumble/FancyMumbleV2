import { Box, Fade, ImageList, ImageListItem, Paper, Popper, Skeleton, TextField } from '@mui/material'
import React, { useState } from 'react';
import SearchIcon from '@mui/icons-material/Search';

interface GifSearchProps {
  open: boolean
  anchor: HTMLElement
}

function GifSearch(props: GifSearchProps) {

  const [search, setSearch] = useState("");
  const [itemData, setItemData] = useState<any[]>([{}, {}, {}, {}, {}, {}]);

  return (
    <Popper open={props.open} anchorEl={props.anchor} transition>
      {({ TransitionProps }) => (
        <Fade {...TransitionProps}>
          <Paper sx={{ p: 1 }}>
            <Box>
              <TextField
                fullWidth
                label="Search Tenor"
                InputProps={{
                  endAdornment: <SearchIcon />,
                }}
                size='small'
                value={search}
                onChange={e => setSearch(e.target.value)}
              />
            </Box>
            <Box>
              <ImageList cols={2} variant="masonry">
                {itemData.map((item) => (
                  <ImageListItem key={item.img}>
                    <Skeleton variant="rectangular" width={180} height={100} />
                  </ImageListItem>
                ))}
              </ImageList>
            </Box>
          </Paper>
        </Fade>
      )}
    </Popper>
  )
}

export default GifSearch