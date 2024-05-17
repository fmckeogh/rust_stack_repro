#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use num_of_ArchVersion::*;
use common::*;
pub fn InitVariantImplemented<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_1036: (),
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_1036: (),
    }
    let fn_state = FunctionState {
        gs_1036,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0u : u32
        let s_0_0: u32 = 0;
        // S s_0_1: call num_of_ArchVersion(s_0_0)
        let s_0_1: i64 = num_of_ArchVersion(state, tracer, s_0_0);
        // C s_0_2: const #100232u : u32
        let s_0_2: u32 = 100232;
        // D s_0_3: read-reg s_0_2:[u8; 16]
        let s_0_3: [bool; 16usize] = {
            let value = state.read_register::<[bool; 16usize]>(s_0_2 as isize);
            tracer.read_register(s_0_2 as isize, value);
            value
        };
        // S s_0_4: cast zx s_0_1 -> i
        let s_0_4: i128 = (i128::try_from(s_0_1).unwrap());
        // C s_0_5: const #11680u : u32
        let s_0_5: u32 = 11680;
        // D s_0_6: read-reg s_0_5:u8
        let s_0_6: bool = {
            let value = state.read_register::<bool>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: mutate-element s_0_3[s_0_4] <= s_0_6
        let s_0_7: [bool; 16usize] = {
            let mut local = s_0_3.clone();
            local[(s_0_4) as usize] = s_0_6;
            local
        };
        // D s_0_8: cast cvt s_0_7 -> [u8; 0]
        let s_0_8: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_0_7);
        // D s_0_9: cast cvt s_0_8 -> [u8; 16]
        let s_0_9: [bool; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_0_8);
            buf
        };
        // C s_0_10: const #100232u : u32
        let s_0_10: u32 = 100232;
        // N s_0_11: write-reg s_0_10 <= s_0_9
        let s_0_11: () = {
            state.write_register::<[bool; 16usize]>(s_0_10 as isize, s_0_9);
            tracer.write_register(s_0_10 as isize, s_0_9);
        };
        // C s_0_12: const #1u : u32
        let s_0_12: u32 = 1;
        // S s_0_13: call num_of_ArchVersion(s_0_12)
        let s_0_13: i64 = num_of_ArchVersion(state, tracer, s_0_12);
        // C s_0_14: const #100232u : u32
        let s_0_14: u32 = 100232;
        // D s_0_15: read-reg s_0_14:[u8; 16]
        let s_0_15: [bool; 16usize] = {
            let value = state.read_register::<[bool; 16usize]>(s_0_14 as isize);
            tracer.read_register(s_0_14 as isize, value);
            value
        };
        // S s_0_16: cast zx s_0_13 -> i
        let s_0_16: i128 = (i128::try_from(s_0_13).unwrap());
        // C s_0_17: const #17640u : u32
        let s_0_17: u32 = 17640;
        // D s_0_18: read-reg s_0_17:u8
        let s_0_18: bool = {
            let value = state.read_register::<bool>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // D s_0_19: mutate-element s_0_15[s_0_16] <= s_0_18
        let s_0_19: [bool; 16usize] = {
            let mut local = s_0_15.clone();
            local[(s_0_16) as usize] = s_0_18;
            local
        };
        // D s_0_20: cast cvt s_0_19 -> [u8; 0]
        let s_0_20: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_0_19);
        // D s_0_21: cast cvt s_0_20 -> [u8; 16]
        let s_0_21: [bool; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_0_20);
            buf
        };
        // C s_0_22: const #100232u : u32
        let s_0_22: u32 = 100232;
        // N s_0_23: write-reg s_0_22 <= s_0_21
        let s_0_23: () = {
            state.write_register::<[bool; 16usize]>(s_0_22 as isize, s_0_21);
            tracer.write_register(s_0_22 as isize, s_0_21);
        };
        // C s_0_24: const #2u : u32
        let s_0_24: u32 = 2;
        // S s_0_25: call num_of_ArchVersion(s_0_24)
        let s_0_25: i64 = num_of_ArchVersion(state, tracer, s_0_24);
        // C s_0_26: const #100232u : u32
        let s_0_26: u32 = 100232;
        // D s_0_27: read-reg s_0_26:[u8; 16]
        let s_0_27: [bool; 16usize] = {
            let value = state.read_register::<[bool; 16usize]>(s_0_26 as isize);
            tracer.read_register(s_0_26 as isize, value);
            value
        };
        // S s_0_28: cast zx s_0_25 -> i
        let s_0_28: i128 = (i128::try_from(s_0_25).unwrap());
        // C s_0_29: const #20864u : u32
        let s_0_29: u32 = 20864;
        // D s_0_30: read-reg s_0_29:u8
        let s_0_30: bool = {
            let value = state.read_register::<bool>(s_0_29 as isize);
            tracer.read_register(s_0_29 as isize, value);
            value
        };
        // D s_0_31: mutate-element s_0_27[s_0_28] <= s_0_30
        let s_0_31: [bool; 16usize] = {
            let mut local = s_0_27.clone();
            local[(s_0_28) as usize] = s_0_30;
            local
        };
        // D s_0_32: cast cvt s_0_31 -> [u8; 0]
        let s_0_32: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_0_31);
        // D s_0_33: cast cvt s_0_32 -> [u8; 16]
        let s_0_33: [bool; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_0_32);
            buf
        };
        // C s_0_34: const #100232u : u32
        let s_0_34: u32 = 100232;
        // N s_0_35: write-reg s_0_34 <= s_0_33
        let s_0_35: () = {
            state.write_register::<[bool; 16usize]>(s_0_34 as isize, s_0_33);
            tracer.write_register(s_0_34 as isize, s_0_33);
        };
        // C s_0_36: const #3u : u32
        let s_0_36: u32 = 3;
        // S s_0_37: call num_of_ArchVersion(s_0_36)
        let s_0_37: i64 = num_of_ArchVersion(state, tracer, s_0_36);
        // C s_0_38: const #100232u : u32
        let s_0_38: u32 = 100232;
        // D s_0_39: read-reg s_0_38:[u8; 16]
        let s_0_39: [bool; 16usize] = {
            let value = state.read_register::<[bool; 16usize]>(s_0_38 as isize);
            tracer.read_register(s_0_38 as isize, value);
            value
        };
        // S s_0_40: cast zx s_0_37 -> i
        let s_0_40: i128 = (i128::try_from(s_0_37).unwrap());
        // C s_0_41: const #15264u : u32
        let s_0_41: u32 = 15264;
        // D s_0_42: read-reg s_0_41:u8
        let s_0_42: bool = {
            let value = state.read_register::<bool>(s_0_41 as isize);
            tracer.read_register(s_0_41 as isize, value);
            value
        };
        // D s_0_43: mutate-element s_0_39[s_0_40] <= s_0_42
        let s_0_43: [bool; 16usize] = {
            let mut local = s_0_39.clone();
            local[(s_0_40) as usize] = s_0_42;
            local
        };
        // D s_0_44: cast cvt s_0_43 -> [u8; 0]
        let s_0_44: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_0_43);
        // D s_0_45: cast cvt s_0_44 -> [u8; 16]
        let s_0_45: [bool; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_0_44);
            buf
        };
        // C s_0_46: const #100232u : u32
        let s_0_46: u32 = 100232;
        // N s_0_47: write-reg s_0_46 <= s_0_45
        let s_0_47: () = {
            state.write_register::<[bool; 16usize]>(s_0_46 as isize, s_0_45);
            tracer.write_register(s_0_46 as isize, s_0_45);
        };
        // C s_0_48: const #4u : u32
        let s_0_48: u32 = 4;
        // S s_0_49: call num_of_ArchVersion(s_0_48)
        let s_0_49: i64 = num_of_ArchVersion(state, tracer, s_0_48);
        // C s_0_50: const #100232u : u32
        let s_0_50: u32 = 100232;
        // D s_0_51: read-reg s_0_50:[u8; 16]
        let s_0_51: [bool; 16usize] = {
            let value = state.read_register::<[bool; 16usize]>(s_0_50 as isize);
            tracer.read_register(s_0_50 as isize, value);
            value
        };
        // S s_0_52: cast zx s_0_49 -> i
        let s_0_52: i128 = (i128::try_from(s_0_49).unwrap());
        // C s_0_53: const #90328u : u32
        let s_0_53: u32 = 90328;
        // D s_0_54: read-reg s_0_53:u8
        let s_0_54: bool = {
            let value = state.read_register::<bool>(s_0_53 as isize);
            tracer.read_register(s_0_53 as isize, value);
            value
        };
        // D s_0_55: mutate-element s_0_51[s_0_52] <= s_0_54
        let s_0_55: [bool; 16usize] = {
            let mut local = s_0_51.clone();
            local[(s_0_52) as usize] = s_0_54;
            local
        };
        // D s_0_56: cast cvt s_0_55 -> [u8; 0]
        let s_0_56: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_0_55);
        // D s_0_57: cast cvt s_0_56 -> [u8; 16]
        let s_0_57: [bool; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_0_56);
            buf
        };
        // C s_0_58: const #100232u : u32
        let s_0_58: u32 = 100232;
        // N s_0_59: write-reg s_0_58 <= s_0_57
        let s_0_59: () = {
            state.write_register::<[bool; 16usize]>(s_0_58 as isize, s_0_57);
            tracer.write_register(s_0_58 as isize, s_0_57);
        };
        // C s_0_60: const #5u : u32
        let s_0_60: u32 = 5;
        // S s_0_61: call num_of_ArchVersion(s_0_60)
        let s_0_61: i64 = num_of_ArchVersion(state, tracer, s_0_60);
        // C s_0_62: const #100232u : u32
        let s_0_62: u32 = 100232;
        // D s_0_63: read-reg s_0_62:[u8; 16]
        let s_0_63: [bool; 16usize] = {
            let value = state.read_register::<[bool; 16usize]>(s_0_62 as isize);
            tracer.read_register(s_0_62 as isize, value);
            value
        };
        // S s_0_64: cast zx s_0_61 -> i
        let s_0_64: i128 = (i128::try_from(s_0_61).unwrap());
        // C s_0_65: const #19128u : u32
        let s_0_65: u32 = 19128;
        // D s_0_66: read-reg s_0_65:u8
        let s_0_66: bool = {
            let value = state.read_register::<bool>(s_0_65 as isize);
            tracer.read_register(s_0_65 as isize, value);
            value
        };
        // D s_0_67: mutate-element s_0_63[s_0_64] <= s_0_66
        let s_0_67: [bool; 16usize] = {
            let mut local = s_0_63.clone();
            local[(s_0_64) as usize] = s_0_66;
            local
        };
        // D s_0_68: cast cvt s_0_67 -> [u8; 0]
        let s_0_68: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_0_67);
        // D s_0_69: cast cvt s_0_68 -> [u8; 16]
        let s_0_69: [bool; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_0_68);
            buf
        };
        // C s_0_70: const #100232u : u32
        let s_0_70: u32 = 100232;
        // N s_0_71: write-reg s_0_70 <= s_0_69
        let s_0_71: () = {
            state.write_register::<[bool; 16usize]>(s_0_70 as isize, s_0_69);
            tracer.write_register(s_0_70 as isize, s_0_69);
        };
        // C s_0_72: const #6u : u32
        let s_0_72: u32 = 6;
        // S s_0_73: call num_of_ArchVersion(s_0_72)
        let s_0_73: i64 = num_of_ArchVersion(state, tracer, s_0_72);
        // C s_0_74: const #100232u : u32
        let s_0_74: u32 = 100232;
        // D s_0_75: read-reg s_0_74:[u8; 16]
        let s_0_75: [bool; 16usize] = {
            let value = state.read_register::<[bool; 16usize]>(s_0_74 as isize);
            tracer.read_register(s_0_74 as isize, value);
            value
        };
        // S s_0_76: cast zx s_0_73 -> i
        let s_0_76: i128 = (i128::try_from(s_0_73).unwrap());
        // C s_0_77: const #20248u : u32
        let s_0_77: u32 = 20248;
        // D s_0_78: read-reg s_0_77:u8
        let s_0_78: bool = {
            let value = state.read_register::<bool>(s_0_77 as isize);
            tracer.read_register(s_0_77 as isize, value);
            value
        };
        // D s_0_79: mutate-element s_0_75[s_0_76] <= s_0_78
        let s_0_79: [bool; 16usize] = {
            let mut local = s_0_75.clone();
            local[(s_0_76) as usize] = s_0_78;
            local
        };
        // D s_0_80: cast cvt s_0_79 -> [u8; 0]
        let s_0_80: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_0_79);
        // D s_0_81: cast cvt s_0_80 -> [u8; 16]
        let s_0_81: [bool; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_0_80);
            buf
        };
        // C s_0_82: const #100232u : u32
        let s_0_82: u32 = 100232;
        // N s_0_83: write-reg s_0_82 <= s_0_81
        let s_0_83: () = {
            state.write_register::<[bool; 16usize]>(s_0_82 as isize, s_0_81);
            tracer.write_register(s_0_82 as isize, s_0_81);
        };
        // C s_0_84: const #7u : u32
        let s_0_84: u32 = 7;
        // S s_0_85: call num_of_ArchVersion(s_0_84)
        let s_0_85: i64 = num_of_ArchVersion(state, tracer, s_0_84);
        // C s_0_86: const #100232u : u32
        let s_0_86: u32 = 100232;
        // D s_0_87: read-reg s_0_86:[u8; 16]
        let s_0_87: [bool; 16usize] = {
            let value = state.read_register::<[bool; 16usize]>(s_0_86 as isize);
            tracer.read_register(s_0_86 as isize, value);
            value
        };
        // S s_0_88: cast zx s_0_85 -> i
        let s_0_88: i128 = (i128::try_from(s_0_85).unwrap());
        // C s_0_89: const #22752u : u32
        let s_0_89: u32 = 22752;
        // D s_0_90: read-reg s_0_89:u8
        let s_0_90: bool = {
            let value = state.read_register::<bool>(s_0_89 as isize);
            tracer.read_register(s_0_89 as isize, value);
            value
        };
        // D s_0_91: mutate-element s_0_87[s_0_88] <= s_0_90
        let s_0_91: [bool; 16usize] = {
            let mut local = s_0_87.clone();
            local[(s_0_88) as usize] = s_0_90;
            local
        };
        // D s_0_92: cast cvt s_0_91 -> [u8; 0]
        let s_0_92: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_0_91);
        // D s_0_93: cast cvt s_0_92 -> [u8; 16]
        let s_0_93: [bool; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_0_92);
            buf
        };
        // C s_0_94: const #100232u : u32
        let s_0_94: u32 = 100232;
        // N s_0_95: write-reg s_0_94 <= s_0_93
        let s_0_95: () = {
            state.write_register::<[bool; 16usize]>(s_0_94 as isize, s_0_93);
            tracer.write_register(s_0_94 as isize, s_0_93);
        };
        // C s_0_96: const #8u : u32
        let s_0_96: u32 = 8;
        // S s_0_97: call num_of_ArchVersion(s_0_96)
        let s_0_97: i64 = num_of_ArchVersion(state, tracer, s_0_96);
        // C s_0_98: const #100232u : u32
        let s_0_98: u32 = 100232;
        // D s_0_99: read-reg s_0_98:[u8; 16]
        let s_0_99: [bool; 16usize] = {
            let value = state.read_register::<[bool; 16usize]>(s_0_98 as isize);
            tracer.read_register(s_0_98 as isize, value);
            value
        };
        // S s_0_100: cast zx s_0_97 -> i
        let s_0_100: i128 = (i128::try_from(s_0_97).unwrap());
        // C s_0_101: const #17704u : u32
        let s_0_101: u32 = 17704;
        // D s_0_102: read-reg s_0_101:u8
        let s_0_102: bool = {
            let value = state.read_register::<bool>(s_0_101 as isize);
            tracer.read_register(s_0_101 as isize, value);
            value
        };
        // D s_0_103: mutate-element s_0_99[s_0_100] <= s_0_102
        let s_0_103: [bool; 16usize] = {
            let mut local = s_0_99.clone();
            local[(s_0_100) as usize] = s_0_102;
            local
        };
        // D s_0_104: cast cvt s_0_103 -> [u8; 0]
        let s_0_104: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_0_103);
        // D s_0_105: cast cvt s_0_104 -> [u8; 16]
        let s_0_105: [bool; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_0_104);
            buf
        };
        // C s_0_106: const #100232u : u32
        let s_0_106: u32 = 100232;
        // N s_0_107: write-reg s_0_106 <= s_0_105
        let s_0_107: () = {
            state.write_register::<[bool; 16usize]>(s_0_106 as isize, s_0_105);
            tracer.write_register(s_0_106 as isize, s_0_105);
        };
        // C s_0_108: const #9u : u32
        let s_0_108: u32 = 9;
        // S s_0_109: call num_of_ArchVersion(s_0_108)
        let s_0_109: i64 = num_of_ArchVersion(state, tracer, s_0_108);
        // C s_0_110: const #100232u : u32
        let s_0_110: u32 = 100232;
        // D s_0_111: read-reg s_0_110:[u8; 16]
        let s_0_111: [bool; 16usize] = {
            let value = state.read_register::<[bool; 16usize]>(s_0_110 as isize);
            tracer.read_register(s_0_110 as isize, value);
            value
        };
        // S s_0_112: cast zx s_0_109 -> i
        let s_0_112: i128 = (i128::try_from(s_0_109).unwrap());
        // C s_0_113: const #101856u : u32
        let s_0_113: u32 = 101856;
        // D s_0_114: read-reg s_0_113:u8
        let s_0_114: bool = {
            let value = state.read_register::<bool>(s_0_113 as isize);
            tracer.read_register(s_0_113 as isize, value);
            value
        };
        // D s_0_115: mutate-element s_0_111[s_0_112] <= s_0_114
        let s_0_115: [bool; 16usize] = {
            let mut local = s_0_111.clone();
            local[(s_0_112) as usize] = s_0_114;
            local
        };
        // D s_0_116: cast cvt s_0_115 -> [u8; 0]
        let s_0_116: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_0_115);
        // D s_0_117: cast cvt s_0_116 -> [u8; 16]
        let s_0_117: [bool; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_0_116);
            buf
        };
        // C s_0_118: const #100232u : u32
        let s_0_118: u32 = 100232;
        // N s_0_119: write-reg s_0_118 <= s_0_117
        let s_0_119: () = {
            state.write_register::<[bool; 16usize]>(s_0_118 as isize, s_0_117);
            tracer.write_register(s_0_118 as isize, s_0_117);
        };
        // C s_0_120: const #10u : u32
        let s_0_120: u32 = 10;
        // S s_0_121: call num_of_ArchVersion(s_0_120)
        let s_0_121: i64 = num_of_ArchVersion(state, tracer, s_0_120);
        // C s_0_122: const #100232u : u32
        let s_0_122: u32 = 100232;
        // D s_0_123: read-reg s_0_122:[u8; 16]
        let s_0_123: [bool; 16usize] = {
            let value = state.read_register::<[bool; 16usize]>(s_0_122 as isize);
            tracer.read_register(s_0_122 as isize, value);
            value
        };
        // S s_0_124: cast zx s_0_121 -> i
        let s_0_124: i128 = (i128::try_from(s_0_121).unwrap());
        // C s_0_125: const #1416u : u32
        let s_0_125: u32 = 1416;
        // D s_0_126: read-reg s_0_125:u8
        let s_0_126: bool = {
            let value = state.read_register::<bool>(s_0_125 as isize);
            tracer.read_register(s_0_125 as isize, value);
            value
        };
        // D s_0_127: mutate-element s_0_123[s_0_124] <= s_0_126
        let s_0_127: [bool; 16usize] = {
            let mut local = s_0_123.clone();
            local[(s_0_124) as usize] = s_0_126;
            local
        };
        // D s_0_128: cast cvt s_0_127 -> [u8; 0]
        let s_0_128: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_0_127);
        // D s_0_129: cast cvt s_0_128 -> [u8; 16]
        let s_0_129: [bool; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_0_128);
            buf
        };
        // C s_0_130: const #100232u : u32
        let s_0_130: u32 = 100232;
        // N s_0_131: write-reg s_0_130 <= s_0_129
        let s_0_131: () = {
            state.write_register::<[bool; 16usize]>(s_0_130 as isize, s_0_129);
            tracer.write_register(s_0_130 as isize, s_0_129);
        };
        // C s_0_132: const #11u : u32
        let s_0_132: u32 = 11;
        // S s_0_133: call num_of_ArchVersion(s_0_132)
        let s_0_133: i64 = num_of_ArchVersion(state, tracer, s_0_132);
        // C s_0_134: const #100232u : u32
        let s_0_134: u32 = 100232;
        // D s_0_135: read-reg s_0_134:[u8; 16]
        let s_0_135: [bool; 16usize] = {
            let value = state.read_register::<[bool; 16usize]>(s_0_134 as isize);
            tracer.read_register(s_0_134 as isize, value);
            value
        };
        // S s_0_136: cast zx s_0_133 -> i
        let s_0_136: i128 = (i128::try_from(s_0_133).unwrap());
        // C s_0_137: const #102760u : u32
        let s_0_137: u32 = 102760;
        // D s_0_138: read-reg s_0_137:u8
        let s_0_138: bool = {
            let value = state.read_register::<bool>(s_0_137 as isize);
            tracer.read_register(s_0_137 as isize, value);
            value
        };
        // D s_0_139: mutate-element s_0_135[s_0_136] <= s_0_138
        let s_0_139: [bool; 16usize] = {
            let mut local = s_0_135.clone();
            local[(s_0_136) as usize] = s_0_138;
            local
        };
        // D s_0_140: cast cvt s_0_139 -> [u8; 0]
        let s_0_140: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_0_139);
        // D s_0_141: cast cvt s_0_140 -> [u8; 16]
        let s_0_141: [bool; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_0_140);
            buf
        };
        // C s_0_142: const #100232u : u32
        let s_0_142: u32 = 100232;
        // N s_0_143: write-reg s_0_142 <= s_0_141
        let s_0_143: () = {
            state.write_register::<[bool; 16usize]>(s_0_142 as isize, s_0_141);
            tracer.write_register(s_0_142 as isize, s_0_141);
        };
        // C s_0_144: const #12u : u32
        let s_0_144: u32 = 12;
        // S s_0_145: call num_of_ArchVersion(s_0_144)
        let s_0_145: i64 = num_of_ArchVersion(state, tracer, s_0_144);
        // C s_0_146: const #100232u : u32
        let s_0_146: u32 = 100232;
        // D s_0_147: read-reg s_0_146:[u8; 16]
        let s_0_147: [bool; 16usize] = {
            let value = state.read_register::<[bool; 16usize]>(s_0_146 as isize);
            tracer.read_register(s_0_146 as isize, value);
            value
        };
        // S s_0_148: cast zx s_0_145 -> i
        let s_0_148: i128 = (i128::try_from(s_0_145).unwrap());
        // C s_0_149: const #90896u : u32
        let s_0_149: u32 = 90896;
        // D s_0_150: read-reg s_0_149:u8
        let s_0_150: bool = {
            let value = state.read_register::<bool>(s_0_149 as isize);
            tracer.read_register(s_0_149 as isize, value);
            value
        };
        // D s_0_151: mutate-element s_0_147[s_0_148] <= s_0_150
        let s_0_151: [bool; 16usize] = {
            let mut local = s_0_147.clone();
            local[(s_0_148) as usize] = s_0_150;
            local
        };
        // D s_0_152: cast cvt s_0_151 -> [u8; 0]
        let s_0_152: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_0_151);
        // D s_0_153: cast cvt s_0_152 -> [u8; 16]
        let s_0_153: [bool; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_0_152);
            buf
        };
        // C s_0_154: const #100232u : u32
        let s_0_154: u32 = 100232;
        // N s_0_155: write-reg s_0_154 <= s_0_153
        let s_0_155: () = {
            state.write_register::<[bool; 16usize]>(s_0_154 as isize, s_0_153);
            tracer.write_register(s_0_154 as isize, s_0_153);
        };
        // C s_0_156: const #13u : u32
        let s_0_156: u32 = 13;
        // S s_0_157: call num_of_ArchVersion(s_0_156)
        let s_0_157: i64 = num_of_ArchVersion(state, tracer, s_0_156);
        // C s_0_158: const #100232u : u32
        let s_0_158: u32 = 100232;
        // D s_0_159: read-reg s_0_158:[u8; 16]
        let s_0_159: [bool; 16usize] = {
            let value = state.read_register::<[bool; 16usize]>(s_0_158 as isize);
            tracer.read_register(s_0_158 as isize, value);
            value
        };
        // S s_0_160: cast zx s_0_157 -> i
        let s_0_160: i128 = (i128::try_from(s_0_157).unwrap());
        // C s_0_161: const #21224u : u32
        let s_0_161: u32 = 21224;
        // D s_0_162: read-reg s_0_161:u8
        let s_0_162: bool = {
            let value = state.read_register::<bool>(s_0_161 as isize);
            tracer.read_register(s_0_161 as isize, value);
            value
        };
        // D s_0_163: mutate-element s_0_159[s_0_160] <= s_0_162
        let s_0_163: [bool; 16usize] = {
            let mut local = s_0_159.clone();
            local[(s_0_160) as usize] = s_0_162;
            local
        };
        // D s_0_164: cast cvt s_0_163 -> [u8; 0]
        let s_0_164: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_0_163);
        // D s_0_165: cast cvt s_0_164 -> [u8; 16]
        let s_0_165: [bool; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_0_164);
            buf
        };
        // C s_0_166: const #100232u : u32
        let s_0_166: u32 = 100232;
        // N s_0_167: write-reg s_0_166 <= s_0_165
        let s_0_167: () = {
            state.write_register::<[bool; 16usize]>(s_0_166 as isize, s_0_165);
            tracer.write_register(s_0_166 as isize, s_0_165);
        };
        // C s_0_168: const #14u : u32
        let s_0_168: u32 = 14;
        // S s_0_169: call num_of_ArchVersion(s_0_168)
        let s_0_169: i64 = num_of_ArchVersion(state, tracer, s_0_168);
        // C s_0_170: const #100232u : u32
        let s_0_170: u32 = 100232;
        // D s_0_171: read-reg s_0_170:[u8; 16]
        let s_0_171: [bool; 16usize] = {
            let value = state.read_register::<[bool; 16usize]>(s_0_170 as isize);
            tracer.read_register(s_0_170 as isize, value);
            value
        };
        // S s_0_172: cast zx s_0_169 -> i
        let s_0_172: i128 = (i128::try_from(s_0_169).unwrap());
        // C s_0_173: const #101792u : u32
        let s_0_173: u32 = 101792;
        // D s_0_174: read-reg s_0_173:u8
        let s_0_174: bool = {
            let value = state.read_register::<bool>(s_0_173 as isize);
            tracer.read_register(s_0_173 as isize, value);
            value
        };
        // D s_0_175: mutate-element s_0_171[s_0_172] <= s_0_174
        let s_0_175: [bool; 16usize] = {
            let mut local = s_0_171.clone();
            local[(s_0_172) as usize] = s_0_174;
            local
        };
        // D s_0_176: cast cvt s_0_175 -> [u8; 0]
        let s_0_176: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_0_175);
        // D s_0_177: cast cvt s_0_176 -> [u8; 16]
        let s_0_177: [bool; 16usize] = {
            let mut buf = [Default::default(); 16usize];
            buf.copy_from_slice(&s_0_176);
            buf
        };
        // C s_0_178: const #100232u : u32
        let s_0_178: u32 = 100232;
        // N s_0_179: write-reg s_0_178 <= s_0_177
        let s_0_179: () = {
            state.write_register::<[bool; 16usize]>(s_0_178 as isize, s_0_177);
            tracer.write_register(s_0_178 as isize, s_0_177);
        };
        // N s_0_180: return
        return;
    }
}
