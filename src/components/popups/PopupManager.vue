@@ .. @@
               <!-- Compress format name presset popup -->
               <PresetFormPopup
                   v-if="popup.component === 'PresetForm'"
                   v-bind="popup.props"
                   @close="hidePopup(popup.id)"
               />

+              <!-- Engine Form Popup -->
+              <EngineFormPopup
+                  v-if="popup.component === 'EngineForm'"
+                  v-bind="popup.props"
+                  @close="hidePopup(popup.id)"
+              />
+
+              <!-- IDE Form Popup -->
+              <IdeFormPopup
+                  v-if="popup.component === 'IdeForm'"
+                  v-bind="popup.props"
+                  @close="hidePopup(popup.id)"
+              />
+
               <!-- Add more popup components here as needed -->
             </div>
           </Transition>
@@ .. @@
 import SettingsPopup from './popups/SettingsPopup.vue'
 import EngineDetectionPopup from './popups/EngineDetectionPopup.vue'
 import PresetFormPopup from "./popups/PresetFormPopup.vue";
+import EngineFormPopup from './popups/EngineFormPopup.vue'
+import IdeFormPopup from './popups/IdeFormPopup.vue'
 
 const {popupState, hidePopup, initPopupListener, showPopup} = usePopup()