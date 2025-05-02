package com.fancy_mumble

import android.os.Bundle
import androidx.core.view.WindowCompat
import android.webkit.WebView
import android.annotation.SuppressLint

class MainActivity : TauriActivity() {
    private lateinit var wv: WebView

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        window.statusBarColor = getColor(R.color.app_bar_color)
    }

    override fun onWebViewCreate(webView: WebView) {
        wv = webView
    }
}