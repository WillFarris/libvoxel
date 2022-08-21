use cgmath::{Vector2, Vector3};

use super::vertex::Vertex3D;

pub(crate) const CUBE_FACES: [[Vertex3D; 6]; 10] = [
    
    // Facing positive-X
    [
        Vertex3D { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },  // Front-bottom-right
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-right
        Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 }, // Front-top-right
    
        Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 }, // Front-top-right
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-right
        Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },  // Back-top-right
    ],

    // Facing negative-X
    [
        Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 }, // Front-top-left
        Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },  // Back-top-left
        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },  // Front-bottom-left
        
        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },  // Front-bottom-left
        Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },  // Back-top-left
        Vertex3D { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-left
    ],

    // Facing positive-Y
    [
        Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
        Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-top-right
        Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left
    
        Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left
        Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-top-right
        Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-top-left
    ],
    
    // Facing negative-Y
    [
        Vertex3D { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-bottom-right
        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-bottom-left
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right

        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-bottom-left
        Vertex3D { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-left
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
    ],

    // Facing positive-Z
    [
        Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
        Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left
        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Front-bottom-left
    
        Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
        Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Front-bottom-left
        Vertex3D { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Front-bottom-right
    ],   

    // Facing negative-Z
    [
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
        Vertex3D { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-left
        Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left
    
        Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
        Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left
        Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 1.0), vtype: 0  }     // Back-top-right
    ],

    // Diagonal (0, 0) -> (1, 1)
    [
        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.146446609407), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.853553390593), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.146446609407, 0.0, 0.146446609407), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },

        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.146446609407), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.99, 0.853553390593), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.853553390593), normal: Vector3::new(-0.701, 0.0, -0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
    ],

    // Diagonal (1, 1) -> (0, 0)
    [
        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.146446609407, 0.0, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },

        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.99, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },
    ],

    // Diagonal (0, 1) -> (1, 0)
    [
        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.146446609407, 0.0, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },

        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.853553390593), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.99, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.146446609407), normal: Vector3::new(0.701, 0.0, 0.701), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
    ],

    // Diagonal (1, 0) -> (0, 1)
    [
        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.853553390593), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.146446609407, 0.0, 0.853553390593), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.146446609407), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },

        Vertex3D { position: Vector3::new(0.146446609407, 0.99, 0.853553390593), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.0, 0.146446609407), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },
        Vertex3D { position: Vector3::new(0.853553390593, 0.99, 0.146446609407), normal: Vector3::new(0.0, 0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0), vtype: 0 },
    ],
];

pub(crate) const DEFAULT_CUBE: [Vertex3D; 36] = [
    // Facing positive-X
    Vertex3D { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },  // Front-bottom-right
    Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 }, // Front-top-right

    Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 }, // Front-top-right
    Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },  // Back-top-right

    // Facing negative-X
    Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 }, // Front-top-left
    Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },  // Back-top-left
    Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },  // Front-bottom-left
    
    Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },  // Front-bottom-left
    Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },  // Back-top-left
    Vertex3D { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( -1.0,  0.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-left

    // Facing positive-Y
    Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
    Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-top-right
    Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left

    Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left
    Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-top-right
    Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  1.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-top-left
    
    // Facing negative-Y
    Vertex3D { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-bottom-right
    Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-bottom-left
    Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right

    Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-bottom-left
    Vertex3D { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-left
    Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  -1.0, 0.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right

    // Facing positive-Z
    Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
    Vertex3D { position: Vector3::new(0.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Front-top-left
    Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Front-bottom-left

    Vertex3D { position: Vector3::new( 1.0,  1.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 1.0) , vtype: 0 },   // Front-top-right
    Vertex3D { position: Vector3::new(0.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Front-bottom-left
    Vertex3D { position: Vector3::new( 1.0, 0.0,  1.0), normal: Vector3::new( 0.0,  0.0,  1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Front-bottom-right

    // Facing negative-Z
    Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex3D { position: Vector3::new(0.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 0.0) , vtype: 0 },   // Back-bottom-left
    Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left

    Vertex3D { position: Vector3::new( 1.0, 0.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 0.0) , vtype: 0 },   // Back-bottom-right
    Vertex3D { position: Vector3::new(0.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(0.0, 1.0) , vtype: 0 },   // Back-top-left
    Vertex3D { position: Vector3::new( 1.0,  1.0, 0.0), normal: Vector3::new( 0.0,  0.0, -1.0), tex_coords: Vector2::new(1.0, 1.0), vtype: 0  }     // Back-top-right

];

pub fn push_face(position: &[f32; 3], face: usize, vertices: &mut Vec<Vertex3D>, texmap_offset: &(f32, f32), vertex_type: i32) {

    for v in 0..6 {
        let mut vertex = CUBE_FACES[face][v];
        vertex.position.x += position[0];
        vertex.position.y += position[1];
        vertex.position.z += position[2];

        vertex.tex_coords.x = vertex.tex_coords.x * 0.0625 + 0.0625 * texmap_offset.0 as f32;
        vertex.tex_coords.y = vertex.tex_coords.y * 0.0625 + 0.0625 * texmap_offset.1 as f32;

        vertex.vtype = vertex_type as i32;

        vertices.push(vertex);
    }
}