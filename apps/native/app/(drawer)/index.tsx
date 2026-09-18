import { Text, View } from "react-native";

import { Container } from "@/components/container";

export default function Home() {
  return (
    <Container className="px-4 pb-4">
      <View className="mb-5 py-6">
        <Text className="text-foreground text-3xl font-semibold tracking-tight">
          Better T Stack
        </Text>
        <Text className="text-muted mt-1 text-sm">
          Full-stack TypeScript starter
        </Text>
      </View>
    </Container>
  );
}
