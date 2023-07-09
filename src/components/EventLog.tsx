import { Box, IconButton, Paper, Table, TableBody, TableCell, TableContainer, TableHead, TableRow, Tooltip } from '@mui/material';


import { RootState } from '../store/store';
import { useSelector } from 'react-redux';
import React, { useEffect, useRef } from 'react';
import dayjs from 'dayjs';

interface EventLogProps {
    showLog: boolean;
}

const EventLog: React.FC<EventLogProps> = React.memo(({ showLog }) => {
    const eventLog = useSelector((state: RootState) => state.eventLog);
    const eventLogRef = useRef<HTMLDivElement | null>(null);

    useEffect(() => {
        if (showLog && eventLogRef.current) {
            eventLogRef.current.scrollTo({ top: eventLogRef.current.scrollHeight, behavior: 'smooth' });
        }
    }, [showLog, eventLog, eventLogRef]);

    if (!showLog) {
        return null;
    }

    return (
        <Box sx={{
            maxWidth: '300px',
            display: 'flex',
            flexDirection: 'column',
            flex: 1,
            overflowY: 'auto',
        }} >
            <Paper
                elevation={0}
                sx={{ flexGrow: 1, overflowX: 'hidden' }}
                ref={eventLogRef}
            >
                <Box sx={{ display: 'flex', flexDirection: 'row', alignItems: 'flex-start', height: '100%' }}>
                    <Box sx={{ flexGrow: 1, paddingLeft: 0 }}>
                        <TableContainer component={Paper}>
                            <Table sx={{ minWidth: 300 }} aria-label="messaeg log" size="small" stickyHeader>
                                <TableHead>
                                    <TableRow>
                                        <TableCell>Timestamp</TableCell>
                                        <TableCell align="left">Message</TableCell>
                                    </TableRow>
                                </TableHead>
                                <TableBody>
                                    {eventLog.map((row, i) => {
                                        const timestamp = dayjs(row.timestamp).format('HH:mm:ss');

                                        return (
                                            <TableRow key={'' + i + row.timestamp} sx={{ '&:last-child td, &:last-child th': { border: 0 } }}>
                                                <TableCell component="th" scope="row">
                                                    {timestamp}
                                                </TableCell>
                                                <TableCell align="left">{row.logMessage}</TableCell>
                                            </TableRow>
                                        )
                                    })}
                                </TableBody>
                            </Table>
                        </TableContainer>
                    </Box>
                </Box>
            </Paper>
        </Box>
    )
});

export default EventLog;