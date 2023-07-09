import { Alert, Box, Container, Divider, Grid, Switch, Typography } from '@mui/material';
import { G } from '@tauri-apps/api/path-e12e0e34';
import React, { useState } from 'react';


interface AdditionalFeaturesProps {
}

const AdditionalFeatures: React.FC<AdditionalFeaturesProps> = React.memo(({ }) => {

    let [errorMessage, setErrorMessage] = useState('');
    const label = { inputProps: { 'aria-label': 'Link Preview' } };

    function showErrorMessage() {
        if (errorMessage) {
            return <Alert severity="error">{errorMessage}</Alert>
        }
    }

    return (
        <Box>
            <Container className="settingsContainer">
                {showErrorMessage()}
                <Typography variant="h4">Additional Features</Typography>
                <Divider sx={{ marginBottom: 5 }} />
                <Grid container spacing={{ xs: 2, md: 3 }} columns={{ xs: 4, sm: 8, md: 12, lg: 18 }} sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
                    <Grid item xs={4} sm={8} md={6} lg={6}>
                        <Typography variant="h6">Enable Link Preview</Typography>
                    </Grid>
                    <Grid item xs={4} sm={8} md={6} lg={6}>
                        <Switch {...label} defaultChecked />
                    </Grid>
                </Grid>
            </Container>
        </Box>
    )
});

export default AdditionalFeatures;