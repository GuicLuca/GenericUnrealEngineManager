@@ .. @@
               <!-- Project Compress Popup -->
               <ProjectCompressPopup
                   v-if="popup.component === 'ProjectCompress'"
                   v-bind="popup.props"
                   @close="hidePopup(popup.id)"
               />

+              <!-- Project Package Popup -->
+              <ProjectPackagePopup
+                  v-if="popup.component === 'ProjectPackage'"
+                  v-bind="popup.props"
+                  @close="hidePopup(popup.id)"
+              />
+
               <!-- Settings Popup -->
               <SettingsPopup
                   v-if="popup.component === 'Settings'"
@@ .. @@
 import ProjectLaunchChoicePopup from './popups/ProjectLaunchChoicePopup.vue'
 import ProjectCleanPopup from './popups/ProjectCleanPopup.vue'
 import ProjectCompressPopup from './popups/ProjectCompressPopup.vue'
+import ProjectPackagePopup from './popups/ProjectPackagePopup.vue'
 import SettingsPopup from './popups/SettingsPopup.vue'