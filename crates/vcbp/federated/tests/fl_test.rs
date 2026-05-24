#[cfg(test)]
mod tests {
    use vcbp_federated::*;

    #[tokio::test]
    async fn test_mesh_init() {
        let mesh = mesh::FlMesh::new(4);
        mesh.start_round().await.unwrap();
    }
}
