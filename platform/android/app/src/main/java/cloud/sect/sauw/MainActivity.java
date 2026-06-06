package cloud.sect.sauw;

import android.os.Bundle;
import android.view.View;
import android.view.Window;
import android.view.WindowManager;

import com.google.androidgamesdk.GameActivity;

public class MainActivity extends GameActivity {
  static {
    System.loadLibrary("sauw");
  }

  @Override
  protected void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);

    configureFullscreenWindow();
    hideSystemUi();
  }

  @Override
  public void onWindowFocusChanged(boolean hasFocus) {
    super.onWindowFocusChanged(hasFocus);

    if (hasFocus) {
      hideSystemUi();
    }
  }

  private void configureFullscreenWindow() {
    Window window = getWindow();
    window.setDecorFitsSystemWindows(false);
    window.addFlags(WindowManager.LayoutParams.FLAG_FULLSCREEN);

    WindowManager.LayoutParams attributes = window.getAttributes();
    attributes.layoutInDisplayCutoutMode =
      WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES;
    window.setAttributes(attributes);
  }

  private void hideSystemUi() {
    View decorView = getWindow().getDecorView();
    decorView.setSystemUiVisibility(
      View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY |
      View.SYSTEM_UI_FLAG_LAYOUT_STABLE |
      View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION |
      View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN |
      View.SYSTEM_UI_FLAG_HIDE_NAVIGATION |
      View.SYSTEM_UI_FLAG_FULLSCREEN
    );
  }
}