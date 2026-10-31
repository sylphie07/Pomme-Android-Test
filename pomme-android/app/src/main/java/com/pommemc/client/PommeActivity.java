package com.pommemc.client;

import android.app.NativeActivity;
import android.content.Intent;
import android.net.Uri;
import android.util.Log;

import java.io.File;
import java.io.FileOutputStream;
import java.io.InputStream;
import java.io.IOException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import android.os.Build;
import android.os.Bundle;
import android.view.View;
import android.view.Window;
import android.view.WindowInsets;
import android.view.WindowInsetsController;

/** NativeActivity wrapper for fullscreen mode and Android's private skin picker bridge. */
public final class PommeActivity extends NativeActivity {
    private static final String TAG = "PommeActivity";
    private static final int REQUEST_SKIN_IMPORT = 0x534B;
    private static final long MAX_SKIN_BYTES = 4L * 1024L * 1024L;
    private static final String PENDING_SKIN_FILE = "pomme-skin-import.pending";
    private static final ExecutorService SKIN_IMPORT_EXECUTOR =
        Executors.newSingleThreadExecutor();

    /** Called from Rust on Android's Java main thread. */
    public void requestSkinImport() {
        deleteFile(PENDING_SKIN_FILE);
        Intent intent = new Intent(Intent.ACTION_OPEN_DOCUMENT);
        intent.addCategory(Intent.CATEGORY_OPENABLE);
        intent.setType("image/png");
        startActivityForResult(intent, REQUEST_SKIN_IMPORT);
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        if (requestCode != REQUEST_SKIN_IMPORT) {
            return;
        }
        if (resultCode != RESULT_OK || data == null || data.getData() == null) {
            return;
        }

        final Uri uri = data.getData();
        final android.content.ContentResolver resolver = getContentResolver();
        final File directory = getFilesDir();
        SKIN_IMPORT_EXECUTOR.execute(() -> copySkinToPrivateStorage(resolver, uri, directory));
    }

    private static void copySkinToPrivateStorage(
        android.content.ContentResolver resolver, Uri uri, File directory
    ) {
        File temporary = new File(directory, PENDING_SKIN_FILE + ".tmp");
        File destination = new File(directory, PENDING_SKIN_FILE);
        try (InputStream input = resolver.openInputStream(uri);
             FileOutputStream output = new FileOutputStream(temporary, false)) {
            if (input == null) {
                throw new IOException("The selected document could not be opened");
            }
            byte[] buffer = new byte[8192];
            long total = 0;
            int count;
            while ((count = input.read(buffer)) != -1) {
                total += count;
                if (total > MAX_SKIN_BYTES) {
                    throw new IOException("The selected skin exceeds the 4 MiB limit");
                }
                output.write(buffer, 0, count);
            }
            if (total == 0) {
                throw new IOException("The selected skin is empty");
            }
            output.flush();
            if (!temporary.renameTo(destination)) {
                throw new IOException("Could not finalize the imported skin file");
            }
        } catch (Exception error) {
            // Never leave a partial result for Rust to consume.
            //noinspection ResultOfMethodCallIgnored
            temporary.delete();
            //noinspection ResultOfMethodCallIgnored
            destination.delete();
            Log.w(TAG, "Skin import failed", error);
        }
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        applyImmersiveMode();
    }

    @Override
    public void onWindowFocusChanged(boolean hasFocus) {
        super.onWindowFocusChanged(hasFocus);
        if (hasFocus) {
            applyImmersiveMode();
        }
    }

    private void applyImmersiveMode() {
        Window window = getWindow();
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
            WindowInsetsController controller = window.getInsetsController();
            if (controller != null) {
                controller.setSystemBarsBehavior(
                    WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
                );
                controller.hide(
                    WindowInsets.Type.statusBars() | WindowInsets.Type.navigationBars()
                );
            }
        } else {
            int flags = View.SYSTEM_UI_FLAG_FULLSCREEN
                | View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
                | View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
                | View.SYSTEM_UI_FLAG_LAYOUT_STABLE
                | View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
                | View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION;
            window.getDecorView().setSystemUiVisibility(flags);
        }
    }
}
