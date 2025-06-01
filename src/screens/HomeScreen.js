import React from 'react';
import { View, Text, StyleSheet } from 'react-native';

const HomeScreen = () => {
  return (
    <View style={styles.container}> {/* Apply container style to the main View */}
      <Text style={styles.title}>Home Screen</Text> {/* Apply title style */}
      <Text style={styles.welcomeText}>Welcome!</Text> {/* Apply welcomeText style */}
      <View style={styles.contentContainer}> {/* Apply contentContainer style */}
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  title: { /* Renamed text style to title for clarity */
    fontSize: 24, 
  },
  welcomeText: {
    fontSize: 18,
    marginTop: 10,
  },
  contentContainer: {
    flex: 1,
    marginTop: 20,
    width: '90%', // Example width, adjust as needed
  },
});

export default HomeScreen;