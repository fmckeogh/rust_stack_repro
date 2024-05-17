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
use MemCpyZeroSizeCheck::*;
use CPYSizeChoice::*;
use CreateAccDescMOPS::*;
use MemCpyParametersIllformedM::*;
use MismatchedMemCpyException::*;
use MemCpyParametersIllformedE::*;
use X_read::*;
use Zeros::*;
use CPYPostSizeChoice::*;
use Mem_set::*;
use neq_int::*;
use X_set::*;
use MaxBlockSizeCopiedBytes::*;
use u__id::*;
use Mem_read::*;
use CPYFOptionA::*;
use AArch64_IsUnprivAccessPriv::*;
use SPESampleMemCopy::*;
use CPYPreSizeChoice::*;
use common::*;
pub fn execute_aarch64_instrs_memory_mcpymset_cpyf<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    n: i64,
    options_name: u8,
    s: i64,
    stage: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_147166: bool,
        raccdesc: ProductType9878976b5bcce9c9,
        toaddress: u64,
        Bshadow_1159: i128,
        ga_251928: Bits,
        gs_147244: bool,
        gs_147256: bool,
        gs_147242: bool,
        rprivileged: bool,
        fromaddress: u64,
        Bshadow_1158: i128,
        waccdesc: ProductType9878976b5bcce9c9,
        gs_147197: bool,
        gs_147229: bool,
        ga_251959: i128,
        ga_251960: Bits,
        readdata: Bits,
        gs_147210: bool,
        gs_147153: bool,
        B: i128,
        Bshadow_1156: i128,
        cpysize: u64,
        Bshadow_1157: i128,
        postsize: u64,
        nzcv: u8,
        ga_251927: i128,
        wprivileged: bool,
        supports_option_a: bool,
        N: i128,
        stagecpysize: u64,
        d: i64,
        n: i64,
        options_name: u8,
        s: i64,
        stage: u32,
    }
    let fn_state = FunctionState {
        d,
        n,
        options_name,
        s,
        stage,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call MaxBlockSizeCopiedBytes(s_0_0)
        let s_0_1: i128 = MaxBlockSizeCopiedBytes(state, tracer, s_0_0);
        // D s_0_2: write-var N <= s_0_1
        fn_state.N = s_0_1;
        // C s_0_3: const #64s : i64
        let s_0_3: i64 = 64;
        // D s_0_4: read-var d:i64
        let s_0_4: i64 = fn_state.d;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: call X_read(s_0_5, s_0_3)
        let s_0_6: Bits = X_read(state, tracer, s_0_5, s_0_3);
        // D s_0_7: cast reint s_0_6 -> u64
        let s_0_7: u64 = (s_0_6.value() as u64);
        // D s_0_8: write-var toaddress <= s_0_7
        fn_state.toaddress = s_0_7;
        // C s_0_9: const #64s : i64
        let s_0_9: i64 = 64;
        // D s_0_10: read-var s:i64
        let s_0_10: i64 = fn_state.s;
        // D s_0_11: cast zx s_0_10 -> i
        let s_0_11: i128 = (i128::try_from(s_0_10).unwrap());
        // D s_0_12: call X_read(s_0_11, s_0_9)
        let s_0_12: Bits = X_read(state, tracer, s_0_11, s_0_9);
        // D s_0_13: cast reint s_0_12 -> u64
        let s_0_13: u64 = (s_0_12.value() as u64);
        // D s_0_14: write-var fromaddress <= s_0_13
        fn_state.fromaddress = s_0_13;
        // C s_0_15: const #64s : i64
        let s_0_15: i64 = 64;
        // D s_0_16: read-var n:i64
        let s_0_16: i64 = fn_state.n;
        // D s_0_17: cast zx s_0_16 -> i
        let s_0_17: i128 = (i128::try_from(s_0_16).unwrap());
        // D s_0_18: call X_read(s_0_17, s_0_15)
        let s_0_18: Bits = X_read(state, tracer, s_0_17, s_0_15);
        // D s_0_19: cast reint s_0_18 -> u64
        let s_0_19: u64 = (s_0_18.value() as u64);
        // D s_0_20: write-var cpysize <= s_0_19
        fn_state.cpysize = s_0_19;
        // C s_0_21: const #16984u : u32
        let s_0_21: u32 = 16984;
        // D s_0_22: read-reg s_0_21:u8
        let s_0_22: bool = {
            let value = state.read_register::<bool>(s_0_21 as isize);
            tracer.read_register(s_0_21 as isize, value);
            value
        };
        // C s_0_23: const #16997u : u32
        let s_0_23: u32 = 16997;
        // D s_0_24: read-reg s_0_23:u8
        let s_0_24: bool = {
            let value = state.read_register::<bool>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // C s_0_25: const #16971u : u32
        let s_0_25: u32 = 16971;
        // D s_0_26: read-reg s_0_25:u8
        let s_0_26: bool = {
            let value = state.read_register::<bool>(s_0_25 as isize);
            tracer.read_register(s_0_25 as isize, value);
            value
        };
        // C s_0_27: const #16996u : u32
        let s_0_27: u32 = 16996;
        // D s_0_28: read-reg s_0_27:u8
        let s_0_28: bool = {
            let value = state.read_register::<bool>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: cast zx s_0_26 -> bv
        let s_0_29: Bits = Bits::new(s_0_26 as u128, 1u16);
        // D s_0_30: cast zx s_0_28 -> bv
        let s_0_30: Bits = Bits::new(s_0_28 as u128, 1u16);
        // D s_0_31: cast reint s_0_29 -> u128
        let s_0_31: u128 = (s_0_29.value() as u128);
        // D s_0_32: size-of s_0_29
        let s_0_32: u16 = s_0_29.length();
        // D s_0_33: cast reint s_0_30 -> u128
        let s_0_33: u128 = (s_0_30.value() as u128);
        // D s_0_34: size-of s_0_30
        let s_0_34: u16 = s_0_30.length();
        // D s_0_35: lsl s_0_31 s_0_34
        let s_0_35: u128 = s_0_31 << s_0_34;
        // D s_0_36: or s_0_35 s_0_33
        let s_0_36: u128 = ((s_0_35) | (s_0_33));
        // D s_0_37: add s_0_32 s_0_34
        let s_0_37: u16 = (s_0_32 + s_0_34);
        // D s_0_38: create-bits s_0_36 s_0_37
        let s_0_38: Bits = Bits::new(s_0_36, s_0_37);
        // D s_0_39: cast reint s_0_38 -> u8
        let s_0_39: u8 = (s_0_38.value() as u8);
        // D s_0_40: cast zx s_0_24 -> bv
        let s_0_40: Bits = Bits::new(s_0_24 as u128, 1u16);
        // D s_0_41: cast zx s_0_39 -> bv
        let s_0_41: Bits = Bits::new(s_0_39 as u128, 2u16);
        // D s_0_42: cast reint s_0_40 -> u128
        let s_0_42: u128 = (s_0_40.value() as u128);
        // D s_0_43: size-of s_0_40
        let s_0_43: u16 = s_0_40.length();
        // D s_0_44: cast reint s_0_41 -> u128
        let s_0_44: u128 = (s_0_41.value() as u128);
        // D s_0_45: size-of s_0_41
        let s_0_45: u16 = s_0_41.length();
        // D s_0_46: lsl s_0_42 s_0_45
        let s_0_46: u128 = s_0_42 << s_0_45;
        // D s_0_47: or s_0_46 s_0_44
        let s_0_47: u128 = ((s_0_46) | (s_0_44));
        // D s_0_48: add s_0_43 s_0_45
        let s_0_48: u16 = (s_0_43 + s_0_45);
        // D s_0_49: create-bits s_0_47 s_0_48
        let s_0_49: Bits = Bits::new(s_0_47, s_0_48);
        // D s_0_50: cast reint s_0_49 -> u8
        let s_0_50: u8 = (s_0_49.value() as u8);
        // D s_0_51: cast zx s_0_22 -> bv
        let s_0_51: Bits = Bits::new(s_0_22 as u128, 1u16);
        // D s_0_52: cast zx s_0_50 -> bv
        let s_0_52: Bits = Bits::new(s_0_50 as u128, 3u16);
        // D s_0_53: cast reint s_0_51 -> u128
        let s_0_53: u128 = (s_0_51.value() as u128);
        // D s_0_54: size-of s_0_51
        let s_0_54: u16 = s_0_51.length();
        // D s_0_55: cast reint s_0_52 -> u128
        let s_0_55: u128 = (s_0_52.value() as u128);
        // D s_0_56: size-of s_0_52
        let s_0_56: u16 = s_0_52.length();
        // D s_0_57: lsl s_0_53 s_0_56
        let s_0_57: u128 = s_0_53 << s_0_56;
        // D s_0_58: or s_0_57 s_0_55
        let s_0_58: u128 = ((s_0_57) | (s_0_55));
        // D s_0_59: add s_0_54 s_0_56
        let s_0_59: u16 = (s_0_54 + s_0_56);
        // D s_0_60: create-bits s_0_58 s_0_59
        let s_0_60: Bits = Bits::new(s_0_58, s_0_59);
        // D s_0_61: cast reint s_0_60 -> u8
        let s_0_61: u8 = (s_0_60.value() as u8);
        // D s_0_62: write-var nzcv <= s_0_61
        fn_state.nzcv = s_0_61;
        // C s_0_63: const #() : ()
        let s_0_63: () = ();
        // S s_0_64: call CPYFOptionA(s_0_63)
        let s_0_64: bool = CPYFOptionA(state, tracer, s_0_63);
        // D s_0_65: write-var supports_option_a <= s_0_64
        fn_state.supports_option_a = s_0_64;
        // C s_0_66: const #1s : i
        let s_0_66: i128 = 1;
        // D s_0_67: read-var options_name:u8
        let s_0_67: u8 = fn_state.options_name;
        // D s_0_68: cast zx s_0_67 -> bv
        let s_0_68: Bits = Bits::new(s_0_67 as u128, 4u16);
        // C s_0_69: const #1u : u64
        let s_0_69: u64 = 1;
        // D s_0_70: bit-extract s_0_68 s_0_66 s_0_69
        let s_0_70: Bits = (Bits::new(
            ((s_0_68) >> (s_0_66)).value(),
            u16::try_from(s_0_69).unwrap(),
        ));
        // D s_0_71: cast reint s_0_70 -> u8
        let s_0_71: bool = ((s_0_70.value()) != 0);
        // C s_0_72: const #0s : i
        let s_0_72: i128 = 0;
        // C s_0_73: const #0u : u64
        let s_0_73: u64 = 0;
        // D s_0_74: cast zx s_0_71 -> u64
        let s_0_74: u64 = (s_0_71 as u64);
        // C s_0_75: const #1u : u64
        let s_0_75: u64 = 1;
        // D s_0_76: and s_0_74 s_0_75
        let s_0_76: u64 = ((s_0_74) & (s_0_75));
        // D s_0_77: cmp-eq s_0_76 s_0_75
        let s_0_77: bool = ((s_0_76) == (s_0_75));
        // D s_0_78: lsl s_0_74 s_0_72
        let s_0_78: u64 = s_0_74 << s_0_72;
        // D s_0_79: or s_0_73 s_0_78
        let s_0_79: u64 = ((s_0_73) | (s_0_78));
        // D s_0_80: cmpl s_0_78
        let s_0_80: u64 = !s_0_78;
        // D s_0_81: and s_0_73 s_0_80
        let s_0_81: u64 = ((s_0_73) & (s_0_80));
        // D s_0_82: select s_0_77 s_0_79 s_0_81
        let s_0_82: u64 = if s_0_77 { s_0_79 } else { s_0_81 };
        // D s_0_83: cast trunc s_0_82 -> u8
        let s_0_83: bool = ((s_0_82) != 0);
        // D s_0_84: cast zx s_0_83 -> bv
        let s_0_84: Bits = Bits::new(s_0_83 as u128, 1u16);
        // C s_0_85: const #1u : u8
        let s_0_85: bool = true;
        // C s_0_86: cast zx s_0_85 -> bv
        let s_0_86: Bits = Bits::new(s_0_85 as u128, 1u16);
        // D s_0_87: cmp-eq s_0_84 s_0_86
        let s_0_87: bool = ((s_0_84) == (s_0_86));
        // N s_0_88: branch s_0_87 b84 b1
        if s_0_87 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16975u : u32
        let s_1_0: u32 = 16975;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 2u16);
        // C s_1_3: const #448u : u32
        let s_1_3: u32 = 448;
        // D s_1_4: read-reg s_1_3:u8
        let s_1_4: u8 = {
            let value = state.read_register::<u8>(s_1_3 as isize);
            tracer.read_register(s_1_3 as isize, value);
            value
        };
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 2u16);
        // D s_1_6: cmp-ne s_1_2 s_1_5
        let s_1_6: bool = ((s_1_2) != (s_1_5));
        // D s_1_7: write-var rprivileged <= s_1_6
        fn_state.rprivileged = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0s : i
        let s_2_0: i128 = 0;
        // D s_2_1: read-var options_name:u8
        let s_2_1: u8 = fn_state.options_name;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 4u16);
        // C s_2_3: const #1u : u64
        let s_2_3: u64 = 1;
        // D s_2_4: bit-extract s_2_2 s_2_0 s_2_3
        let s_2_4: Bits = (Bits::new(
            ((s_2_2) >> (s_2_0)).value(),
            u16::try_from(s_2_3).unwrap(),
        ));
        // D s_2_5: cast reint s_2_4 -> u8
        let s_2_5: bool = ((s_2_4.value()) != 0);
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // C s_2_7: const #0u : u64
        let s_2_7: u64 = 0;
        // D s_2_8: cast zx s_2_5 -> u64
        let s_2_8: u64 = (s_2_5 as u64);
        // C s_2_9: const #1u : u64
        let s_2_9: u64 = 1;
        // D s_2_10: and s_2_8 s_2_9
        let s_2_10: u64 = ((s_2_8) & (s_2_9));
        // D s_2_11: cmp-eq s_2_10 s_2_9
        let s_2_11: bool = ((s_2_10) == (s_2_9));
        // D s_2_12: lsl s_2_8 s_2_6
        let s_2_12: u64 = s_2_8 << s_2_6;
        // D s_2_13: or s_2_7 s_2_12
        let s_2_13: u64 = ((s_2_7) | (s_2_12));
        // D s_2_14: cmpl s_2_12
        let s_2_14: u64 = !s_2_12;
        // D s_2_15: and s_2_7 s_2_14
        let s_2_15: u64 = ((s_2_7) & (s_2_14));
        // D s_2_16: select s_2_11 s_2_13 s_2_15
        let s_2_16: u64 = if s_2_11 { s_2_13 } else { s_2_15 };
        // D s_2_17: cast trunc s_2_16 -> u8
        let s_2_17: bool = ((s_2_16) != 0);
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 1u16);
        // C s_2_19: const #1u : u8
        let s_2_19: bool = true;
        // C s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 1u16);
        // D s_2_21: cmp-eq s_2_18 s_2_20
        let s_2_21: bool = ((s_2_18) == (s_2_20));
        // N s_2_22: branch s_2_21 b83 b3
        if s_2_21 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #16975u : u32
        let s_3_0: u32 = 16975;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 2u16);
        // C s_3_3: const #448u : u32
        let s_3_3: u32 = 448;
        // D s_3_4: read-reg s_3_3:u8
        let s_3_4: u8 = {
            let value = state.read_register::<u8>(s_3_3 as isize);
            tracer.read_register(s_3_3 as isize, value);
            value
        };
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 2u16);
        // D s_3_6: cmp-ne s_3_2 s_3_5
        let s_3_6: bool = ((s_3_2) != (s_3_5));
        // D s_3_7: write-var wprivileged <= s_3_6
        fn_state.wprivileged = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #3s : i
        let s_4_0: i128 = 3;
        // D s_4_1: read-var options_name:u8
        let s_4_1: u8 = fn_state.options_name;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 4u16);
        // C s_4_3: const #1u : u64
        let s_4_3: u64 = 1;
        // D s_4_4: bit-extract s_4_2 s_4_0 s_4_3
        let s_4_4: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_3).unwrap(),
        ));
        // D s_4_5: cast reint s_4_4 -> u8
        let s_4_5: bool = ((s_4_4.value()) != 0);
        // C s_4_6: const #0s : i
        let s_4_6: i128 = 0;
        // C s_4_7: const #0u : u64
        let s_4_7: u64 = 0;
        // D s_4_8: cast zx s_4_5 -> u64
        let s_4_8: u64 = (s_4_5 as u64);
        // C s_4_9: const #1u : u64
        let s_4_9: u64 = 1;
        // D s_4_10: and s_4_8 s_4_9
        let s_4_10: u64 = ((s_4_8) & (s_4_9));
        // D s_4_11: cmp-eq s_4_10 s_4_9
        let s_4_11: bool = ((s_4_10) == (s_4_9));
        // D s_4_12: lsl s_4_8 s_4_6
        let s_4_12: u64 = s_4_8 << s_4_6;
        // D s_4_13: or s_4_7 s_4_12
        let s_4_13: u64 = ((s_4_7) | (s_4_12));
        // D s_4_14: cmpl s_4_12
        let s_4_14: u64 = !s_4_12;
        // D s_4_15: and s_4_7 s_4_14
        let s_4_15: u64 = ((s_4_7) & (s_4_14));
        // D s_4_16: select s_4_11 s_4_13 s_4_15
        let s_4_16: u64 = if s_4_11 { s_4_13 } else { s_4_15 };
        // D s_4_17: cast trunc s_4_16 -> u8
        let s_4_17: bool = ((s_4_16) != 0);
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 1u16);
        // C s_4_19: const #1u : u8
        let s_4_19: bool = true;
        // C s_4_20: cast zx s_4_19 -> bv
        let s_4_20: Bits = Bits::new(s_4_19 as u128, 1u16);
        // D s_4_21: cmp-eq s_4_18 s_4_20
        let s_4_21: bool = ((s_4_18) == (s_4_20));
        // C s_4_22: const #2s : i
        let s_4_22: i128 = 2;
        // D s_4_23: read-var options_name:u8
        let s_4_23: u8 = fn_state.options_name;
        // D s_4_24: cast zx s_4_23 -> bv
        let s_4_24: Bits = Bits::new(s_4_23 as u128, 4u16);
        // C s_4_25: const #1u : u64
        let s_4_25: u64 = 1;
        // D s_4_26: bit-extract s_4_24 s_4_22 s_4_25
        let s_4_26: Bits = (Bits::new(
            ((s_4_24) >> (s_4_22)).value(),
            u16::try_from(s_4_25).unwrap(),
        ));
        // D s_4_27: cast reint s_4_26 -> u8
        let s_4_27: bool = ((s_4_26.value()) != 0);
        // C s_4_28: const #0s : i
        let s_4_28: i128 = 0;
        // C s_4_29: const #0u : u64
        let s_4_29: u64 = 0;
        // D s_4_30: cast zx s_4_27 -> u64
        let s_4_30: u64 = (s_4_27 as u64);
        // C s_4_31: const #1u : u64
        let s_4_31: u64 = 1;
        // D s_4_32: and s_4_30 s_4_31
        let s_4_32: u64 = ((s_4_30) & (s_4_31));
        // D s_4_33: cmp-eq s_4_32 s_4_31
        let s_4_33: bool = ((s_4_32) == (s_4_31));
        // D s_4_34: lsl s_4_30 s_4_28
        let s_4_34: u64 = s_4_30 << s_4_28;
        // D s_4_35: or s_4_29 s_4_34
        let s_4_35: u64 = ((s_4_29) | (s_4_34));
        // D s_4_36: cmpl s_4_34
        let s_4_36: u64 = !s_4_34;
        // D s_4_37: and s_4_29 s_4_36
        let s_4_37: u64 = ((s_4_29) & (s_4_36));
        // D s_4_38: select s_4_33 s_4_35 s_4_37
        let s_4_38: u64 = if s_4_33 { s_4_35 } else { s_4_37 };
        // D s_4_39: cast trunc s_4_38 -> u8
        let s_4_39: bool = ((s_4_38) != 0);
        // D s_4_40: cast zx s_4_39 -> bv
        let s_4_40: Bits = Bits::new(s_4_39 as u128, 1u16);
        // C s_4_41: const #1u : u8
        let s_4_41: bool = true;
        // C s_4_42: cast zx s_4_41 -> bv
        let s_4_42: Bits = Bits::new(s_4_41 as u128, 1u16);
        // D s_4_43: cmp-eq s_4_40 s_4_42
        let s_4_43: bool = ((s_4_40) == (s_4_42));
        // C s_4_44: const #0u : u32
        let s_4_44: u32 = 0;
        // D s_4_45: read-var rprivileged:u8
        let s_4_45: bool = fn_state.rprivileged;
        // D s_4_46: call CreateAccDescMOPS(s_4_44, s_4_45, s_4_21)
        let s_4_46: ProductType9878976b5bcce9c9 = CreateAccDescMOPS(
            state,
            tracer,
            s_4_44,
            s_4_45,
            s_4_21,
        );
        // D s_4_47: write-var raccdesc <= s_4_46
        fn_state.raccdesc = s_4_46;
        // C s_4_48: const #1u : u32
        let s_4_48: u32 = 1;
        // D s_4_49: read-var wprivileged:u8
        let s_4_49: bool = fn_state.wprivileged;
        // D s_4_50: call CreateAccDescMOPS(s_4_48, s_4_49, s_4_43)
        let s_4_50: ProductType9878976b5bcce9c9 = CreateAccDescMOPS(
            state,
            tracer,
            s_4_48,
            s_4_49,
            s_4_43,
        );
        // D s_4_51: write-var waccdesc <= s_4_50
        fn_state.waccdesc = s_4_50;
        // D s_4_52: read-var stage:u32
        let s_4_52: u32 = fn_state.stage;
        // C s_4_53: const #0u : u32
        let s_4_53: u32 = 0;
        // D s_4_54: cmp-eq s_4_52 s_4_53
        let s_4_54: bool = ((s_4_52) == (s_4_53));
        // N s_4_55: branch s_4_54 b71 b5
        if s_4_54 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call MemCpyZeroSizeCheck(s_5_0)
        let s_5_1: bool = MemCpyZeroSizeCheck(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b70 b6
        if s_5_1 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var cpysize:u64
        let s_6_0: u64 = fn_state.cpysize;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 64u16);
        // D s_6_2: cast sx s_6_1 -> i
        let s_6_2: i128 = {
            let sign_bit = s_6_1.length() - 1;
            let mut result = s_6_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #0s : i
        let s_6_4: i128 = 0;
        // D s_6_5: cast zx s_6_3 -> i
        let s_6_5: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_6: call neq_int(s_6_5, s_6_4)
        let s_6_6: bool = neq_int(state, tracer, s_6_5, s_6_4);
        // D s_6_7: write-var gs#147229 <= s_6_6
        fn_state.gs_147229 = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#147229:u8
        let s_7_0: bool = fn_state.gs_147229;
        // N s_7_1: branch s_7_0 b61 b8
        if s_7_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var toaddress:u64
        let s_9_0: u64 = fn_state.toaddress;
        // D s_9_1: read-var fromaddress:u64
        let s_9_1: u64 = fn_state.fromaddress;
        // D s_9_2: read-var cpysize:u64
        let s_9_2: u64 = fn_state.cpysize;
        // D s_9_3: call CPYPostSizeChoice(s_9_0, s_9_1, s_9_2)
        let s_9_3: u64 = CPYPostSizeChoice(state, tracer, s_9_0, s_9_1, s_9_2);
        // D s_9_4: write-var postsize <= s_9_3
        fn_state.postsize = s_9_3;
        // C s_9_5: const #63s : i
        let s_9_5: i128 = 63;
        // D s_9_6: read-var postsize:u64
        let s_9_6: u64 = fn_state.postsize;
        // D s_9_7: cast zx s_9_6 -> bv
        let s_9_7: Bits = Bits::new(s_9_6 as u128, 64u16);
        // C s_9_8: const #1u : u64
        let s_9_8: u64 = 1;
        // D s_9_9: bit-extract s_9_7 s_9_5 s_9_8
        let s_9_9: Bits = (Bits::new(
            ((s_9_7) >> (s_9_5)).value(),
            u16::try_from(s_9_8).unwrap(),
        ));
        // D s_9_10: cast reint s_9_9 -> u8
        let s_9_10: bool = ((s_9_9.value()) != 0);
        // C s_9_11: const #0s : i
        let s_9_11: i128 = 0;
        // C s_9_12: const #0u : u64
        let s_9_12: u64 = 0;
        // D s_9_13: cast zx s_9_10 -> u64
        let s_9_13: u64 = (s_9_10 as u64);
        // C s_9_14: const #1u : u64
        let s_9_14: u64 = 1;
        // D s_9_15: and s_9_13 s_9_14
        let s_9_15: u64 = ((s_9_13) & (s_9_14));
        // D s_9_16: cmp-eq s_9_15 s_9_14
        let s_9_16: bool = ((s_9_15) == (s_9_14));
        // D s_9_17: lsl s_9_13 s_9_11
        let s_9_17: u64 = s_9_13 << s_9_11;
        // D s_9_18: or s_9_12 s_9_17
        let s_9_18: u64 = ((s_9_12) | (s_9_17));
        // D s_9_19: cmpl s_9_17
        let s_9_19: u64 = !s_9_17;
        // D s_9_20: and s_9_12 s_9_19
        let s_9_20: u64 = ((s_9_12) & (s_9_19));
        // D s_9_21: select s_9_16 s_9_18 s_9_20
        let s_9_21: u64 = if s_9_16 { s_9_18 } else { s_9_20 };
        // D s_9_22: cast trunc s_9_21 -> u8
        let s_9_22: bool = ((s_9_21) != 0);
        // C s_9_23: const #63s : i
        let s_9_23: i128 = 63;
        // D s_9_24: read-var cpysize:u64
        let s_9_24: u64 = fn_state.cpysize;
        // D s_9_25: cast zx s_9_24 -> bv
        let s_9_25: Bits = Bits::new(s_9_24 as u128, 64u16);
        // C s_9_26: const #1u : u64
        let s_9_26: u64 = 1;
        // D s_9_27: bit-extract s_9_25 s_9_23 s_9_26
        let s_9_27: Bits = (Bits::new(
            ((s_9_25) >> (s_9_23)).value(),
            u16::try_from(s_9_26).unwrap(),
        ));
        // D s_9_28: cast reint s_9_27 -> u8
        let s_9_28: bool = ((s_9_27.value()) != 0);
        // C s_9_29: const #0s : i
        let s_9_29: i128 = 0;
        // C s_9_30: const #0u : u64
        let s_9_30: u64 = 0;
        // D s_9_31: cast zx s_9_28 -> u64
        let s_9_31: u64 = (s_9_28 as u64);
        // C s_9_32: const #1u : u64
        let s_9_32: u64 = 1;
        // D s_9_33: and s_9_31 s_9_32
        let s_9_33: u64 = ((s_9_31) & (s_9_32));
        // D s_9_34: cmp-eq s_9_33 s_9_32
        let s_9_34: bool = ((s_9_33) == (s_9_32));
        // D s_9_35: lsl s_9_31 s_9_29
        let s_9_35: u64 = s_9_31 << s_9_29;
        // D s_9_36: or s_9_30 s_9_35
        let s_9_36: u64 = ((s_9_30) | (s_9_35));
        // D s_9_37: cmpl s_9_35
        let s_9_37: u64 = !s_9_35;
        // D s_9_38: and s_9_30 s_9_37
        let s_9_38: u64 = ((s_9_30) & (s_9_37));
        // D s_9_39: select s_9_34 s_9_36 s_9_38
        let s_9_39: u64 = if s_9_34 { s_9_36 } else { s_9_38 };
        // D s_9_40: cast trunc s_9_39 -> u8
        let s_9_40: bool = ((s_9_39) != 0);
        // D s_9_41: cast zx s_9_22 -> bv
        let s_9_41: Bits = Bits::new(s_9_22 as u128, 1u16);
        // D s_9_42: cast zx s_9_40 -> bv
        let s_9_42: Bits = Bits::new(s_9_40 as u128, 1u16);
        // D s_9_43: cmp-eq s_9_41 s_9_42
        let s_9_43: bool = ((s_9_41) == (s_9_42));
        // N s_9_44: branch s_9_43 b60 b10
        if s_9_43 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var postsize:u64
        let s_10_0: u64 = fn_state.postsize;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 64u16);
        // D s_10_2: cast sx s_10_1 -> i
        let s_10_2: i128 = {
            let sign_bit = s_10_1.length() - 1;
            let mut result = s_10_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // C s_10_4: const #0s : i
        let s_10_4: i128 = 0;
        // D s_10_5: cast zx s_10_3 -> i
        let s_10_5: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_6: cmp-eq s_10_5 s_10_4
        let s_10_6: bool = ((s_10_5) == (s_10_4));
        // D s_10_7: write-var gs#147242 <= s_10_6
        fn_state.gs_147242 = s_10_6;
        // N s_10_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#147242:u8
        let s_11_0: bool = fn_state.gs_147242;
        // N s_11_1: assert s_11_0
        let s_11_1: () = assert!(s_11_0);
        // D s_11_2: read-var stage:u32
        let s_11_2: u32 = fn_state.stage;
        // C s_11_3: const #1u : u32
        let s_11_3: u32 = 1;
        // D s_11_4: cmp-eq s_11_2 s_11_3
        let s_11_4: bool = ((s_11_2) == (s_11_3));
        // N s_11_5: branch s_11_4 b56 b12
        if s_11_4 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var postsize:u64
        let s_12_0: u64 = fn_state.postsize;
        // D s_12_1: write-var stagecpysize <= s_12_0
        fn_state.stagecpysize = s_12_0;
        // D s_12_2: read-var cpysize:u64
        let s_12_2: u64 = fn_state.cpysize;
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 64u16);
        // D s_12_4: read-var postsize:u64
        let s_12_4: u64 = fn_state.postsize;
        // D s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 64u16);
        // D s_12_6: cmp-ne s_12_3 s_12_5
        let s_12_6: bool = ((s_12_3) != (s_12_5));
        // N s_12_7: branch s_12_6 b55 b13
        if s_12_6 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var toaddress:u64
        let s_13_0: u64 = fn_state.toaddress;
        // D s_13_1: read-var fromaddress:u64
        let s_13_1: u64 = fn_state.fromaddress;
        // D s_13_2: read-var cpysize:u64
        let s_13_2: u64 = fn_state.cpysize;
        // D s_13_3: call MemCpyParametersIllformedE(s_13_0, s_13_1, s_13_2)
        let s_13_3: bool = MemCpyParametersIllformedE(
            state,
            tracer,
            s_13_0,
            s_13_1,
            s_13_2,
        );
        // D s_13_4: write-var gs#147244 <= s_13_3
        fn_state.gs_147244 = s_13_3;
        // N s_13_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#147244:u8
        let s_14_0: bool = fn_state.gs_147244;
        // N s_14_1: branch s_14_0 b54 b15
        if s_14_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #22416u : u32
        let s_17_0: u32 = 22416;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: bool = {
            let value = state.read_register::<bool>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // N s_17_2: branch s_17_1 b53 b18
        if s_17_1 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var supports_option_a:u8
        let s_19_0: bool = fn_state.supports_option_a;
        // N s_19_1: branch s_19_0 b38 b20
        if s_19_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var stagecpysize:u64
        let s_21_0: u64 = fn_state.stagecpysize;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 64u16);
        // D s_21_2: cast zx s_21_1 -> i
        let s_21_2: i128 = (s_21_1.value() as i128);
        // C s_21_3: const #0s : i
        let s_21_3: i128 = 0;
        // D s_21_4: cmp-gt s_21_2 s_21_3
        let s_21_4: bool = ((s_21_2) > (s_21_3));
        // D s_21_5: not s_21_4
        let s_21_5: bool = !s_21_4;
        // N s_21_6: branch s_21_5 b34 b22
        if s_21_5 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var toaddress:u64
        let s_22_0: u64 = fn_state.toaddress;
        // D s_22_1: read-var fromaddress:u64
        let s_22_1: u64 = fn_state.fromaddress;
        // D s_22_2: read-var cpysize:u64
        let s_22_2: u64 = fn_state.cpysize;
        // D s_22_3: call CPYSizeChoice(s_22_0, s_22_1, s_22_2)
        let s_22_3: i128 = CPYSizeChoice(state, tracer, s_22_0, s_22_1, s_22_2);
        // D s_22_4: write-var B <= s_22_3
        fn_state.B = s_22_3;
        // D s_22_5: read-var stagecpysize:u64
        let s_22_5: u64 = fn_state.stagecpysize;
        // D s_22_6: cast zx s_22_5 -> bv
        let s_22_6: Bits = Bits::new(s_22_5 as u128, 64u16);
        // D s_22_7: cast zx s_22_6 -> i
        let s_22_7: i128 = (s_22_6.value() as i128);
        // D s_22_8: read-var B:i
        let s_22_8: i128 = fn_state.B;
        // D s_22_9: cmp-le s_22_8 s_22_7
        let s_22_9: bool = ((s_22_8) <= (s_22_7));
        // N s_22_10: assert s_22_9
        let s_22_10: () = assert!(s_22_9);
        // D s_22_11: read-var B:i
        let s_22_11: i128 = fn_state.B;
        // D s_22_12: write-var Bshadow#1156 <= s_22_11
        fn_state.Bshadow_1156 = s_22_11;
        // D s_22_13: read-var Bshadow#1156:i
        let s_22_13: i128 = fn_state.Bshadow_1156;
        // D s_22_14: call __id(s_22_13)
        let s_22_14: i128 = u__id(state, tracer, s_22_13);
        // C s_22_15: const #8s : i
        let s_22_15: i128 = 8;
        // D s_22_16: mul s_22_14 s_22_15
        let s_22_16: i128 = ((s_22_14) * (s_22_15));
        // C s_22_17: const #1s : i
        let s_22_17: i128 = 1;
        // D s_22_18: sub s_22_16 s_22_17
        let s_22_18: i128 = ((s_22_16) - (s_22_17));
        // C s_22_19: const #0s : i
        let s_22_19: i128 = 0;
        // D s_22_20: cmp-le s_22_19 s_22_18
        let s_22_20: bool = ((s_22_19) <= (s_22_18));
        // N s_22_21: branch s_22_20 b33 b23
        if s_22_20 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#147153 <= s_23_0
        fn_state.gs_147153 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#147153:u8
        let s_24_0: bool = fn_state.gs_147153;
        // N s_24_1: assert s_24_0
        let s_24_1: () = assert!(s_24_0);
        // C s_24_2: const #8s : i
        let s_24_2: i128 = 8;
        // D s_24_3: read-var Bshadow#1156:i
        let s_24_3: i128 = fn_state.Bshadow_1156;
        // D s_24_4: mul s_24_3 s_24_2
        let s_24_4: i128 = ((s_24_3) * (s_24_2));
        // C s_24_5: const #1s : i
        let s_24_5: i128 = 1;
        // D s_24_6: sub s_24_4 s_24_5
        let s_24_6: i128 = ((s_24_4) - (s_24_5));
        // D s_24_7: write-var ga#251959 <= s_24_6
        fn_state.ga_251959 = s_24_6;
        // D s_24_8: read-var fromaddress:u64
        let s_24_8: u64 = fn_state.fromaddress;
        // D s_24_9: read-var Bshadow#1156:i
        let s_24_9: i128 = fn_state.Bshadow_1156;
        // D s_24_10: read-var raccdesc:struct
        let s_24_10: ProductType9878976b5bcce9c9 = fn_state.raccdesc;
        // D s_24_11: call Mem_read(s_24_8, s_24_9, s_24_10)
        let s_24_11: Bits = Mem_read(state, tracer, s_24_8, s_24_9, s_24_10);
        // D s_24_12: write-var ga#251960 <= s_24_11
        fn_state.ga_251960 = s_24_11;
        // N s_24_13: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0s : i
        let s_25_0: i128 = 0;
        // D s_25_1: read-var readdata:bv
        let s_25_1: Bits = fn_state.readdata;
        // D s_25_2: read-var ga#251959:i
        let s_25_2: i128 = fn_state.ga_251959;
        // D s_25_3: read-var ga#251960:bv
        let s_25_3: Bits = fn_state.ga_251960;
        // D s_25_4: sub s_25_2 s_25_0
        let s_25_4: i128 = ((s_25_2) - (s_25_0));
        // C s_25_5: const #1u : u64
        let s_25_5: u64 = 1;
        // C s_25_6: cast zx s_25_5 -> bv
        let s_25_6: Bits = Bits::new(s_25_5 as u128, 64u16);
        // D s_25_7: lsl s_25_6 s_25_4
        let s_25_7: Bits = s_25_6 << s_25_4;
        // D s_25_8: sub s_25_7 s_25_6
        let s_25_8: Bits = ((s_25_7) - (s_25_6));
        // D s_25_9: and s_25_3 s_25_8
        let s_25_9: Bits = ((s_25_3) & (s_25_8));
        // D s_25_10: lsl s_25_9 s_25_0
        let s_25_10: Bits = s_25_9 << s_25_0;
        // D s_25_11: lsl s_25_8 s_25_0
        let s_25_11: Bits = s_25_8 << s_25_0;
        // D s_25_12: cmpl s_25_11
        let s_25_12: Bits = !s_25_11;
        // D s_25_13: and s_25_1 s_25_12
        let s_25_13: Bits = ((s_25_1) & (s_25_12));
        // D s_25_14: or s_25_13 s_25_10
        let s_25_14: Bits = ((s_25_13) | (s_25_10));
        // D s_25_15: write-var readdata <= s_25_14
        fn_state.readdata = s_25_14;
        // D s_25_16: read-var B:i
        let s_25_16: i128 = fn_state.B;
        // D s_25_17: write-var Bshadow#1157 <= s_25_16
        fn_state.Bshadow_1157 = s_25_16;
        // D s_25_18: read-var Bshadow#1157:i
        let s_25_18: i128 = fn_state.Bshadow_1157;
        // D s_25_19: call __id(s_25_18)
        let s_25_19: i128 = u__id(state, tracer, s_25_18);
        // C s_25_20: const #8s : i
        let s_25_20: i128 = 8;
        // D s_25_21: mul s_25_19 s_25_20
        let s_25_21: i128 = ((s_25_19) * (s_25_20));
        // C s_25_22: const #1s : i
        let s_25_22: i128 = 1;
        // D s_25_23: sub s_25_21 s_25_22
        let s_25_23: i128 = ((s_25_21) - (s_25_22));
        // C s_25_24: const #0s : i
        let s_25_24: i128 = 0;
        // D s_25_25: cmp-le s_25_24 s_25_23
        let s_25_25: bool = ((s_25_24) <= (s_25_23));
        // N s_25_26: branch s_25_25 b32 b26
        if s_25_25 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#147166 <= s_26_0
        fn_state.gs_147166 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#147166:u8
        let s_27_0: bool = fn_state.gs_147166;
        // N s_27_1: assert s_27_0
        let s_27_1: () = assert!(s_27_0);
        // C s_27_2: const #8s : i
        let s_27_2: i128 = 8;
        // D s_27_3: read-var Bshadow#1157:i
        let s_27_3: i128 = fn_state.Bshadow_1157;
        // D s_27_4: mul s_27_3 s_27_2
        let s_27_4: i128 = ((s_27_3) * (s_27_2));
        // C s_27_5: const #1s : i
        let s_27_5: i128 = 1;
        // D s_27_6: sub s_27_4 s_27_5
        let s_27_6: i128 = ((s_27_4) - (s_27_5));
        // C s_27_7: const #0s : i
        let s_27_7: i128 = 0;
        // D s_27_8: read-var readdata:bv
        let s_27_8: Bits = fn_state.readdata;
        // C s_27_9: const #1s : i64
        let s_27_9: i64 = 1;
        // C s_27_10: cast zx s_27_9 -> i
        let s_27_10: i128 = (i128::try_from(s_27_9).unwrap());
        // D s_27_11: sub s_27_6 s_27_7
        let s_27_11: i128 = ((s_27_6) - (s_27_7));
        // D s_27_12: add s_27_11 s_27_10
        let s_27_12: i128 = (s_27_11 + s_27_10);
        // D s_27_13: bit-extract s_27_8 s_27_7 s_27_12
        let s_27_13: Bits = (Bits::new(
            ((s_27_8) >> (s_27_7)).value(),
            u16::try_from(s_27_12).unwrap(),
        ));
        // D s_27_14: read-var toaddress:u64
        let s_27_14: u64 = fn_state.toaddress;
        // D s_27_15: read-var Bshadow#1157:i
        let s_27_15: i128 = fn_state.Bshadow_1157;
        // D s_27_16: read-var waccdesc:struct
        let s_27_16: ProductType9878976b5bcce9c9 = fn_state.waccdesc;
        // D s_27_17: call Mem_set(s_27_14, s_27_15, s_27_16, s_27_13)
        let s_27_17: () = Mem_set(state, tracer, s_27_14, s_27_15, s_27_16, s_27_13);
        // N s_27_18: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var fromaddress:u64
        let s_28_0: u64 = fn_state.fromaddress;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 64u16);
        // D s_28_2: read-var B:i
        let s_28_2: i128 = fn_state.B;
        // D s_28_3: cast cvt s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 128);
        // D s_28_4: add s_28_1 s_28_3
        let s_28_4: Bits = (s_28_1 + s_28_3);
        // D s_28_5: cast reint s_28_4 -> u64
        let s_28_5: u64 = (s_28_4.value() as u64);
        // D s_28_6: write-var fromaddress <= s_28_5
        fn_state.fromaddress = s_28_5;
        // D s_28_7: read-var toaddress:u64
        let s_28_7: u64 = fn_state.toaddress;
        // D s_28_8: cast zx s_28_7 -> bv
        let s_28_8: Bits = Bits::new(s_28_7 as u128, 64u16);
        // D s_28_9: read-var B:i
        let s_28_9: i128 = fn_state.B;
        // D s_28_10: cast cvt s_28_9 -> bv
        let s_28_10: Bits = Bits::new(s_28_9 as u128, 128);
        // D s_28_11: add s_28_8 s_28_10
        let s_28_11: Bits = (s_28_8 + s_28_10);
        // D s_28_12: cast reint s_28_11 -> u64
        let s_28_12: u64 = (s_28_11.value() as u64);
        // D s_28_13: write-var toaddress <= s_28_12
        fn_state.toaddress = s_28_12;
        // D s_28_14: read-var cpysize:u64
        let s_28_14: u64 = fn_state.cpysize;
        // D s_28_15: cast zx s_28_14 -> bv
        let s_28_15: Bits = Bits::new(s_28_14 as u128, 64u16);
        // D s_28_16: read-var B:i
        let s_28_16: i128 = fn_state.B;
        // D s_28_17: cast cvt s_28_16 -> bv
        let s_28_17: Bits = Bits::new(s_28_16 as u128, 128);
        // D s_28_18: sub s_28_15 s_28_17
        let s_28_18: Bits = ((s_28_15) - (s_28_17));
        // D s_28_19: cast reint s_28_18 -> u64
        let s_28_19: u64 = (s_28_18.value() as u64);
        // D s_28_20: write-var cpysize <= s_28_19
        fn_state.cpysize = s_28_19;
        // D s_28_21: read-var stagecpysize:u64
        let s_28_21: u64 = fn_state.stagecpysize;
        // D s_28_22: cast zx s_28_21 -> bv
        let s_28_22: Bits = Bits::new(s_28_21 as u128, 64u16);
        // D s_28_23: read-var B:i
        let s_28_23: i128 = fn_state.B;
        // D s_28_24: cast cvt s_28_23 -> bv
        let s_28_24: Bits = Bits::new(s_28_23 as u128, 128);
        // D s_28_25: sub s_28_22 s_28_24
        let s_28_25: Bits = ((s_28_22) - (s_28_24));
        // D s_28_26: cast reint s_28_25 -> u64
        let s_28_26: u64 = (s_28_25.value() as u64);
        // D s_28_27: write-var stagecpysize <= s_28_26
        fn_state.stagecpysize = s_28_26;
        // D s_28_28: read-var stage:u32
        let s_28_28: u32 = fn_state.stage;
        // C s_28_29: const #0u : u32
        let s_28_29: u32 = 0;
        // D s_28_30: cmp-eq s_28_28 s_28_29
        let s_28_30: bool = ((s_28_28) == (s_28_29));
        // N s_28_31: branch s_28_30 b31 b29
        if s_28_30 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_30_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #64s : i64
        let s_31_0: i64 = 64;
        // D s_31_1: read-var n:i64
        let s_31_1: i64 = fn_state.n;
        // D s_31_2: cast zx s_31_1 -> i
        let s_31_2: i128 = (i128::try_from(s_31_1).unwrap());
        // D s_31_3: read-var cpysize:u64
        let s_31_3: u64 = fn_state.cpysize;
        // D s_31_4: cast zx s_31_3 -> bv
        let s_31_4: Bits = Bits::new(s_31_3 as u128, 64u16);
        // D s_31_5: call X_set(s_31_2, s_31_0, s_31_4)
        let s_31_5: () = X_set(state, tracer, s_31_2, s_31_0, s_31_4);
        // C s_31_6: const #64s : i64
        let s_31_6: i64 = 64;
        // D s_31_7: read-var d:i64
        let s_31_7: i64 = fn_state.d;
        // D s_31_8: cast zx s_31_7 -> i
        let s_31_8: i128 = (i128::try_from(s_31_7).unwrap());
        // D s_31_9: read-var toaddress:u64
        let s_31_9: u64 = fn_state.toaddress;
        // D s_31_10: cast zx s_31_9 -> bv
        let s_31_10: Bits = Bits::new(s_31_9 as u128, 64u16);
        // D s_31_11: call X_set(s_31_8, s_31_6, s_31_10)
        let s_31_11: () = X_set(state, tracer, s_31_8, s_31_6, s_31_10);
        // C s_31_12: const #64s : i64
        let s_31_12: i64 = 64;
        // D s_31_13: read-var s:i64
        let s_31_13: i64 = fn_state.s;
        // D s_31_14: cast zx s_31_13 -> i
        let s_31_14: i128 = (i128::try_from(s_31_13).unwrap());
        // D s_31_15: read-var fromaddress:u64
        let s_31_15: u64 = fn_state.fromaddress;
        // D s_31_16: cast zx s_31_15 -> bv
        let s_31_16: Bits = Bits::new(s_31_15 as u128, 64u16);
        // D s_31_17: call X_set(s_31_14, s_31_12, s_31_16)
        let s_31_17: () = X_set(state, tracer, s_31_14, s_31_12, s_31_16);
        // N s_31_18: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var Bshadow#1157:i
        let s_32_0: i128 = fn_state.Bshadow_1157;
        // D s_32_1: call __id(s_32_0)
        let s_32_1: i128 = u__id(state, tracer, s_32_0);
        // C s_32_2: const #8s : i
        let s_32_2: i128 = 8;
        // D s_32_3: mul s_32_1 s_32_2
        let s_32_3: i128 = ((s_32_1) * (s_32_2));
        // C s_32_4: const #1s : i
        let s_32_4: i128 = 1;
        // D s_32_5: sub s_32_3 s_32_4
        let s_32_5: i128 = ((s_32_3) - (s_32_4));
        // D s_32_6: read-var N:i
        let s_32_6: i128 = fn_state.N;
        // D s_32_7: call __id(s_32_6)
        let s_32_7: i128 = u__id(state, tracer, s_32_6);
        // C s_32_8: const #8s : i
        let s_32_8: i128 = 8;
        // D s_32_9: mul s_32_8 s_32_7
        let s_32_9: i128 = ((s_32_8) * (s_32_7));
        // D s_32_10: cmp-lt s_32_5 s_32_9
        let s_32_10: bool = ((s_32_5) < (s_32_9));
        // D s_32_11: write-var gs#147166 <= s_32_10
        fn_state.gs_147166 = s_32_10;
        // N s_32_12: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var Bshadow#1156:i
        let s_33_0: i128 = fn_state.Bshadow_1156;
        // D s_33_1: call __id(s_33_0)
        let s_33_1: i128 = u__id(state, tracer, s_33_0);
        // C s_33_2: const #8s : i
        let s_33_2: i128 = 8;
        // D s_33_3: mul s_33_1 s_33_2
        let s_33_3: i128 = ((s_33_1) * (s_33_2));
        // C s_33_4: const #1s : i
        let s_33_4: i128 = 1;
        // D s_33_5: sub s_33_3 s_33_4
        let s_33_5: i128 = ((s_33_3) - (s_33_4));
        // D s_33_6: read-var N:i
        let s_33_6: i128 = fn_state.N;
        // D s_33_7: call __id(s_33_6)
        let s_33_7: i128 = u__id(state, tracer, s_33_6);
        // C s_33_8: const #8s : i
        let s_33_8: i128 = 8;
        // D s_33_9: mul s_33_8 s_33_7
        let s_33_9: i128 = ((s_33_8) * (s_33_7));
        // D s_33_10: cmp-lt s_33_5 s_33_9
        let s_33_10: bool = ((s_33_5) < (s_33_9));
        // D s_33_11: write-var gs#147153 <= s_33_10
        fn_state.gs_147153 = s_33_10;
        // N s_33_12: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var stage:u32
        let s_35_0: u32 = fn_state.stage;
        // C s_35_1: const #0u : u32
        let s_35_1: u32 = 0;
        // D s_35_2: cmp-eq s_35_0 s_35_1
        let s_35_2: bool = ((s_35_0) == (s_35_1));
        // N s_35_3: branch s_35_2 b37 b36
        if s_35_2 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #64s : i64
        let s_37_0: i64 = 64;
        // D s_37_1: read-var n:i64
        let s_37_1: i64 = fn_state.n;
        // D s_37_2: cast zx s_37_1 -> i
        let s_37_2: i128 = (i128::try_from(s_37_1).unwrap());
        // D s_37_3: read-var cpysize:u64
        let s_37_3: u64 = fn_state.cpysize;
        // D s_37_4: cast zx s_37_3 -> bv
        let s_37_4: Bits = Bits::new(s_37_3 as u128, 64u16);
        // D s_37_5: call X_set(s_37_2, s_37_0, s_37_4)
        let s_37_5: () = X_set(state, tracer, s_37_2, s_37_0, s_37_4);
        // C s_37_6: const #64s : i64
        let s_37_6: i64 = 64;
        // D s_37_7: read-var d:i64
        let s_37_7: i64 = fn_state.d;
        // D s_37_8: cast zx s_37_7 -> i
        let s_37_8: i128 = (i128::try_from(s_37_7).unwrap());
        // D s_37_9: read-var toaddress:u64
        let s_37_9: u64 = fn_state.toaddress;
        // D s_37_10: cast zx s_37_9 -> bv
        let s_37_10: Bits = Bits::new(s_37_9 as u128, 64u16);
        // D s_37_11: call X_set(s_37_8, s_37_6, s_37_10)
        let s_37_11: () = X_set(state, tracer, s_37_8, s_37_6, s_37_10);
        // C s_37_12: const #64s : i64
        let s_37_12: i64 = 64;
        // D s_37_13: read-var s:i64
        let s_37_13: i64 = fn_state.s;
        // D s_37_14: cast zx s_37_13 -> i
        let s_37_14: i128 = (i128::try_from(s_37_13).unwrap());
        // D s_37_15: read-var fromaddress:u64
        let s_37_15: u64 = fn_state.fromaddress;
        // D s_37_16: cast zx s_37_15 -> bv
        let s_37_16: Bits = Bits::new(s_37_15 as u128, 64u16);
        // D s_37_17: call X_set(s_37_14, s_37_12, s_37_16)
        let s_37_17: () = X_set(state, tracer, s_37_14, s_37_12, s_37_16);
        // C s_37_18: const #3s : i
        let s_37_18: i128 = 3;
        // D s_37_19: read-var nzcv:u8
        let s_37_19: u8 = fn_state.nzcv;
        // D s_37_20: cast zx s_37_19 -> bv
        let s_37_20: Bits = Bits::new(s_37_19 as u128, 4u16);
        // C s_37_21: const #1s : i64
        let s_37_21: i64 = 1;
        // C s_37_22: cast zx s_37_21 -> i
        let s_37_22: i128 = (i128::try_from(s_37_21).unwrap());
        // C s_37_23: const #0s : i
        let s_37_23: i128 = 0;
        // C s_37_24: add s_37_23 s_37_22
        let s_37_24: i128 = (s_37_23 + s_37_22);
        // D s_37_25: bit-extract s_37_20 s_37_18 s_37_24
        let s_37_25: Bits = (Bits::new(
            ((s_37_20) >> (s_37_18)).value(),
            u16::try_from(s_37_24).unwrap(),
        ));
        // D s_37_26: cast reint s_37_25 -> u8
        let s_37_26: bool = ((s_37_25.value()) != 0);
        // C s_37_27: const #16984u : u32
        let s_37_27: u32 = 16984;
        // N s_37_28: write-reg s_37_27 <= s_37_26
        let s_37_28: () = {
            state.write_register::<bool>(s_37_27 as isize, s_37_26);
            tracer.write_register(s_37_27 as isize, s_37_26);
        };
        // C s_37_29: const #2s : i
        let s_37_29: i128 = 2;
        // D s_37_30: read-var nzcv:u8
        let s_37_30: u8 = fn_state.nzcv;
        // D s_37_31: cast zx s_37_30 -> bv
        let s_37_31: Bits = Bits::new(s_37_30 as u128, 4u16);
        // C s_37_32: const #1s : i64
        let s_37_32: i64 = 1;
        // C s_37_33: cast zx s_37_32 -> i
        let s_37_33: i128 = (i128::try_from(s_37_32).unwrap());
        // C s_37_34: const #0s : i
        let s_37_34: i128 = 0;
        // C s_37_35: add s_37_34 s_37_33
        let s_37_35: i128 = (s_37_34 + s_37_33);
        // D s_37_36: bit-extract s_37_31 s_37_29 s_37_35
        let s_37_36: Bits = (Bits::new(
            ((s_37_31) >> (s_37_29)).value(),
            u16::try_from(s_37_35).unwrap(),
        ));
        // D s_37_37: cast reint s_37_36 -> u8
        let s_37_37: bool = ((s_37_36.value()) != 0);
        // C s_37_38: const #16997u : u32
        let s_37_38: u32 = 16997;
        // N s_37_39: write-reg s_37_38 <= s_37_37
        let s_37_39: () = {
            state.write_register::<bool>(s_37_38 as isize, s_37_37);
            tracer.write_register(s_37_38 as isize, s_37_37);
        };
        // C s_37_40: const #1s : i
        let s_37_40: i128 = 1;
        // D s_37_41: read-var nzcv:u8
        let s_37_41: u8 = fn_state.nzcv;
        // D s_37_42: cast zx s_37_41 -> bv
        let s_37_42: Bits = Bits::new(s_37_41 as u128, 4u16);
        // C s_37_43: const #1s : i64
        let s_37_43: i64 = 1;
        // C s_37_44: cast zx s_37_43 -> i
        let s_37_44: i128 = (i128::try_from(s_37_43).unwrap());
        // C s_37_45: const #0s : i
        let s_37_45: i128 = 0;
        // C s_37_46: add s_37_45 s_37_44
        let s_37_46: i128 = (s_37_45 + s_37_44);
        // D s_37_47: bit-extract s_37_42 s_37_40 s_37_46
        let s_37_47: Bits = (Bits::new(
            ((s_37_42) >> (s_37_40)).value(),
            u16::try_from(s_37_46).unwrap(),
        ));
        // D s_37_48: cast reint s_37_47 -> u8
        let s_37_48: bool = ((s_37_47.value()) != 0);
        // C s_37_49: const #16971u : u32
        let s_37_49: u32 = 16971;
        // N s_37_50: write-reg s_37_49 <= s_37_48
        let s_37_50: () = {
            state.write_register::<bool>(s_37_49 as isize, s_37_48);
            tracer.write_register(s_37_49 as isize, s_37_48);
        };
        // C s_37_51: const #0s : i
        let s_37_51: i128 = 0;
        // D s_37_52: read-var nzcv:u8
        let s_37_52: u8 = fn_state.nzcv;
        // D s_37_53: cast zx s_37_52 -> bv
        let s_37_53: Bits = Bits::new(s_37_52 as u128, 4u16);
        // C s_37_54: const #1s : i64
        let s_37_54: i64 = 1;
        // C s_37_55: cast zx s_37_54 -> i
        let s_37_55: i128 = (i128::try_from(s_37_54).unwrap());
        // C s_37_56: const #0s : i
        let s_37_56: i128 = 0;
        // C s_37_57: add s_37_56 s_37_55
        let s_37_57: i128 = (s_37_56 + s_37_55);
        // D s_37_58: bit-extract s_37_53 s_37_51 s_37_57
        let s_37_58: Bits = (Bits::new(
            ((s_37_53) >> (s_37_51)).value(),
            u16::try_from(s_37_57).unwrap(),
        ));
        // D s_37_59: cast reint s_37_58 -> u8
        let s_37_59: bool = ((s_37_58.value()) != 0);
        // C s_37_60: const #16996u : u32
        let s_37_60: u32 = 16996;
        // N s_37_61: write-reg s_37_60 <= s_37_59
        let s_37_61: () = {
            state.write_register::<bool>(s_37_60 as isize, s_37_59);
            tracer.write_register(s_37_60 as isize, s_37_59);
        };
        // N s_37_62: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_38_0: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var stagecpysize:u64
        let s_39_0: u64 = fn_state.stagecpysize;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 64u16);
        // D s_39_2: cast sx s_39_1 -> i
        let s_39_2: i128 = {
            let sign_bit = s_39_1.length() - 1;
            let mut result = s_39_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_39_3: cast reint s_39_2 -> i64
        let s_39_3: i64 = (s_39_2 as i64);
        // C s_39_4: const #0s : i
        let s_39_4: i128 = 0;
        // D s_39_5: cast zx s_39_3 -> i
        let s_39_5: i128 = (i128::try_from(s_39_3).unwrap());
        // D s_39_6: call neq_int(s_39_5, s_39_4)
        let s_39_6: bool = neq_int(state, tracer, s_39_5, s_39_4);
        // D s_39_7: not s_39_6
        let s_39_7: bool = !s_39_6;
        // N s_39_8: branch s_39_7 b52 b40
        if s_39_7 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var toaddress:u64
        let s_40_0: u64 = fn_state.toaddress;
        // D s_40_1: read-var fromaddress:u64
        let s_40_1: u64 = fn_state.fromaddress;
        // D s_40_2: read-var cpysize:u64
        let s_40_2: u64 = fn_state.cpysize;
        // D s_40_3: call CPYSizeChoice(s_40_0, s_40_1, s_40_2)
        let s_40_3: i128 = CPYSizeChoice(state, tracer, s_40_0, s_40_1, s_40_2);
        // D s_40_4: write-var B <= s_40_3
        fn_state.B = s_40_3;
        // C s_40_5: const #1s : i
        let s_40_5: i128 = 1;
        // C s_40_6: neg s_40_5
        let s_40_6: i128 = -s_40_5;
        // C s_40_7: cast reint s_40_6 -> i64
        let s_40_7: i64 = (s_40_6 as i64);
        // D s_40_8: read-var stagecpysize:u64
        let s_40_8: u64 = fn_state.stagecpysize;
        // D s_40_9: cast zx s_40_8 -> bv
        let s_40_9: Bits = Bits::new(s_40_8 as u128, 64u16);
        // D s_40_10: cast sx s_40_9 -> i
        let s_40_10: i128 = {
            let sign_bit = s_40_9.length() - 1;
            let mut result = s_40_9.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_40_11: cast reint s_40_10 -> i64
        let s_40_11: i64 = (s_40_10 as i64);
        // C s_40_12: cast zx s_40_7 -> i
        let s_40_12: i128 = (i128::try_from(s_40_7).unwrap());
        // D s_40_13: cast zx s_40_11 -> i
        let s_40_13: i128 = (i128::try_from(s_40_11).unwrap());
        // D s_40_14: mul s_40_12 s_40_13
        let s_40_14: i128 = ((s_40_12) * (s_40_13));
        // D s_40_15: read-var B:i
        let s_40_15: i128 = fn_state.B;
        // D s_40_16: cmp-le s_40_15 s_40_14
        let s_40_16: bool = ((s_40_15) <= (s_40_14));
        // N s_40_17: assert s_40_16
        let s_40_17: () = assert!(s_40_16);
        // D s_40_18: read-var B:i
        let s_40_18: i128 = fn_state.B;
        // D s_40_19: write-var Bshadow#1158 <= s_40_18
        fn_state.Bshadow_1158 = s_40_18;
        // D s_40_20: read-var Bshadow#1158:i
        let s_40_20: i128 = fn_state.Bshadow_1158;
        // D s_40_21: call __id(s_40_20)
        let s_40_21: i128 = u__id(state, tracer, s_40_20);
        // C s_40_22: const #8s : i
        let s_40_22: i128 = 8;
        // D s_40_23: mul s_40_21 s_40_22
        let s_40_23: i128 = ((s_40_21) * (s_40_22));
        // C s_40_24: const #1s : i
        let s_40_24: i128 = 1;
        // D s_40_25: sub s_40_23 s_40_24
        let s_40_25: i128 = ((s_40_23) - (s_40_24));
        // C s_40_26: const #0s : i
        let s_40_26: i128 = 0;
        // D s_40_27: cmp-le s_40_26 s_40_25
        let s_40_27: bool = ((s_40_26) <= (s_40_25));
        // N s_40_28: branch s_40_27 b51 b41
        if s_40_27 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#147197 <= s_41_0
        fn_state.gs_147197 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#147197:u8
        let s_42_0: bool = fn_state.gs_147197;
        // N s_42_1: assert s_42_0
        let s_42_1: () = assert!(s_42_0);
        // C s_42_2: const #8s : i
        let s_42_2: i128 = 8;
        // D s_42_3: read-var Bshadow#1158:i
        let s_42_3: i128 = fn_state.Bshadow_1158;
        // D s_42_4: mul s_42_3 s_42_2
        let s_42_4: i128 = ((s_42_3) * (s_42_2));
        // C s_42_5: const #1s : i
        let s_42_5: i128 = 1;
        // D s_42_6: sub s_42_4 s_42_5
        let s_42_6: i128 = ((s_42_4) - (s_42_5));
        // D s_42_7: write-var ga#251927 <= s_42_6
        fn_state.ga_251927 = s_42_6;
        // D s_42_8: read-var fromaddress:u64
        let s_42_8: u64 = fn_state.fromaddress;
        // D s_42_9: cast zx s_42_8 -> bv
        let s_42_9: Bits = Bits::new(s_42_8 as u128, 64u16);
        // D s_42_10: read-var cpysize:u64
        let s_42_10: u64 = fn_state.cpysize;
        // D s_42_11: cast zx s_42_10 -> bv
        let s_42_11: Bits = Bits::new(s_42_10 as u128, 64u16);
        // D s_42_12: add s_42_9 s_42_11
        let s_42_12: Bits = (s_42_9 + s_42_11);
        // D s_42_13: cast reint s_42_12 -> u64
        let s_42_13: u64 = (s_42_12.value() as u64);
        // D s_42_14: read-var Bshadow#1158:i
        let s_42_14: i128 = fn_state.Bshadow_1158;
        // D s_42_15: read-var raccdesc:struct
        let s_42_15: ProductType9878976b5bcce9c9 = fn_state.raccdesc;
        // D s_42_16: call Mem_read(s_42_13, s_42_14, s_42_15)
        let s_42_16: Bits = Mem_read(state, tracer, s_42_13, s_42_14, s_42_15);
        // D s_42_17: write-var ga#251928 <= s_42_16
        fn_state.ga_251928 = s_42_16;
        // N s_42_18: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0s : i
        let s_43_0: i128 = 0;
        // D s_43_1: read-var readdata:bv
        let s_43_1: Bits = fn_state.readdata;
        // D s_43_2: read-var ga#251927:i
        let s_43_2: i128 = fn_state.ga_251927;
        // D s_43_3: read-var ga#251928:bv
        let s_43_3: Bits = fn_state.ga_251928;
        // D s_43_4: sub s_43_2 s_43_0
        let s_43_4: i128 = ((s_43_2) - (s_43_0));
        // C s_43_5: const #1u : u64
        let s_43_5: u64 = 1;
        // C s_43_6: cast zx s_43_5 -> bv
        let s_43_6: Bits = Bits::new(s_43_5 as u128, 64u16);
        // D s_43_7: lsl s_43_6 s_43_4
        let s_43_7: Bits = s_43_6 << s_43_4;
        // D s_43_8: sub s_43_7 s_43_6
        let s_43_8: Bits = ((s_43_7) - (s_43_6));
        // D s_43_9: and s_43_3 s_43_8
        let s_43_9: Bits = ((s_43_3) & (s_43_8));
        // D s_43_10: lsl s_43_9 s_43_0
        let s_43_10: Bits = s_43_9 << s_43_0;
        // D s_43_11: lsl s_43_8 s_43_0
        let s_43_11: Bits = s_43_8 << s_43_0;
        // D s_43_12: cmpl s_43_11
        let s_43_12: Bits = !s_43_11;
        // D s_43_13: and s_43_1 s_43_12
        let s_43_13: Bits = ((s_43_1) & (s_43_12));
        // D s_43_14: or s_43_13 s_43_10
        let s_43_14: Bits = ((s_43_13) | (s_43_10));
        // D s_43_15: write-var readdata <= s_43_14
        fn_state.readdata = s_43_14;
        // D s_43_16: read-var B:i
        let s_43_16: i128 = fn_state.B;
        // D s_43_17: write-var Bshadow#1159 <= s_43_16
        fn_state.Bshadow_1159 = s_43_16;
        // D s_43_18: read-var Bshadow#1159:i
        let s_43_18: i128 = fn_state.Bshadow_1159;
        // D s_43_19: call __id(s_43_18)
        let s_43_19: i128 = u__id(state, tracer, s_43_18);
        // C s_43_20: const #8s : i
        let s_43_20: i128 = 8;
        // D s_43_21: mul s_43_19 s_43_20
        let s_43_21: i128 = ((s_43_19) * (s_43_20));
        // C s_43_22: const #1s : i
        let s_43_22: i128 = 1;
        // D s_43_23: sub s_43_21 s_43_22
        let s_43_23: i128 = ((s_43_21) - (s_43_22));
        // C s_43_24: const #0s : i
        let s_43_24: i128 = 0;
        // D s_43_25: cmp-le s_43_24 s_43_23
        let s_43_25: bool = ((s_43_24) <= (s_43_23));
        // N s_43_26: branch s_43_25 b50 b44
        if s_43_25 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#147210 <= s_44_0
        fn_state.gs_147210 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#147210:u8
        let s_45_0: bool = fn_state.gs_147210;
        // N s_45_1: assert s_45_0
        let s_45_1: () = assert!(s_45_0);
        // D s_45_2: read-var toaddress:u64
        let s_45_2: u64 = fn_state.toaddress;
        // D s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 64u16);
        // D s_45_4: read-var cpysize:u64
        let s_45_4: u64 = fn_state.cpysize;
        // D s_45_5: cast zx s_45_4 -> bv
        let s_45_5: Bits = Bits::new(s_45_4 as u128, 64u16);
        // D s_45_6: add s_45_3 s_45_5
        let s_45_6: Bits = (s_45_3 + s_45_5);
        // D s_45_7: cast reint s_45_6 -> u64
        let s_45_7: u64 = (s_45_6.value() as u64);
        // C s_45_8: const #8s : i
        let s_45_8: i128 = 8;
        // D s_45_9: read-var Bshadow#1159:i
        let s_45_9: i128 = fn_state.Bshadow_1159;
        // D s_45_10: mul s_45_9 s_45_8
        let s_45_10: i128 = ((s_45_9) * (s_45_8));
        // C s_45_11: const #1s : i
        let s_45_11: i128 = 1;
        // D s_45_12: sub s_45_10 s_45_11
        let s_45_12: i128 = ((s_45_10) - (s_45_11));
        // C s_45_13: const #0s : i
        let s_45_13: i128 = 0;
        // D s_45_14: read-var readdata:bv
        let s_45_14: Bits = fn_state.readdata;
        // C s_45_15: const #1s : i64
        let s_45_15: i64 = 1;
        // C s_45_16: cast zx s_45_15 -> i
        let s_45_16: i128 = (i128::try_from(s_45_15).unwrap());
        // D s_45_17: sub s_45_12 s_45_13
        let s_45_17: i128 = ((s_45_12) - (s_45_13));
        // D s_45_18: add s_45_17 s_45_16
        let s_45_18: i128 = (s_45_17 + s_45_16);
        // D s_45_19: bit-extract s_45_14 s_45_13 s_45_18
        let s_45_19: Bits = (Bits::new(
            ((s_45_14) >> (s_45_13)).value(),
            u16::try_from(s_45_18).unwrap(),
        ));
        // D s_45_20: read-var Bshadow#1159:i
        let s_45_20: i128 = fn_state.Bshadow_1159;
        // D s_45_21: read-var waccdesc:struct
        let s_45_21: ProductType9878976b5bcce9c9 = fn_state.waccdesc;
        // D s_45_22: call Mem_set(s_45_7, s_45_20, s_45_21, s_45_19)
        let s_45_22: () = Mem_set(state, tracer, s_45_7, s_45_20, s_45_21, s_45_19);
        // N s_45_23: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var cpysize:u64
        let s_46_0: u64 = fn_state.cpysize;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 64u16);
        // D s_46_2: read-var B:i
        let s_46_2: i128 = fn_state.B;
        // D s_46_3: cast cvt s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 128);
        // D s_46_4: add s_46_1 s_46_3
        let s_46_4: Bits = (s_46_1 + s_46_3);
        // D s_46_5: cast reint s_46_4 -> u64
        let s_46_5: u64 = (s_46_4.value() as u64);
        // D s_46_6: write-var cpysize <= s_46_5
        fn_state.cpysize = s_46_5;
        // D s_46_7: read-var stagecpysize:u64
        let s_46_7: u64 = fn_state.stagecpysize;
        // D s_46_8: cast zx s_46_7 -> bv
        let s_46_8: Bits = Bits::new(s_46_7 as u128, 64u16);
        // D s_46_9: read-var B:i
        let s_46_9: i128 = fn_state.B;
        // D s_46_10: cast cvt s_46_9 -> bv
        let s_46_10: Bits = Bits::new(s_46_9 as u128, 128);
        // D s_46_11: add s_46_8 s_46_10
        let s_46_11: Bits = (s_46_8 + s_46_10);
        // D s_46_12: cast reint s_46_11 -> u64
        let s_46_12: u64 = (s_46_11.value() as u64);
        // D s_46_13: write-var stagecpysize <= s_46_12
        fn_state.stagecpysize = s_46_12;
        // D s_46_14: read-var stage:u32
        let s_46_14: u32 = fn_state.stage;
        // C s_46_15: const #0u : u32
        let s_46_15: u32 = 0;
        // D s_46_16: cmp-eq s_46_14 s_46_15
        let s_46_16: bool = ((s_46_14) == (s_46_15));
        // N s_46_17: branch s_46_16 b49 b47
        if s_46_16 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_47_0: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_48_0: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #64s : i64
        let s_49_0: i64 = 64;
        // D s_49_1: read-var n:i64
        let s_49_1: i64 = fn_state.n;
        // D s_49_2: cast zx s_49_1 -> i
        let s_49_2: i128 = (i128::try_from(s_49_1).unwrap());
        // D s_49_3: read-var cpysize:u64
        let s_49_3: u64 = fn_state.cpysize;
        // D s_49_4: cast zx s_49_3 -> bv
        let s_49_4: Bits = Bits::new(s_49_3 as u128, 64u16);
        // D s_49_5: call X_set(s_49_2, s_49_0, s_49_4)
        let s_49_5: () = X_set(state, tracer, s_49_2, s_49_0, s_49_4);
        // N s_49_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var Bshadow#1159:i
        let s_50_0: i128 = fn_state.Bshadow_1159;
        // D s_50_1: call __id(s_50_0)
        let s_50_1: i128 = u__id(state, tracer, s_50_0);
        // C s_50_2: const #8s : i
        let s_50_2: i128 = 8;
        // D s_50_3: mul s_50_1 s_50_2
        let s_50_3: i128 = ((s_50_1) * (s_50_2));
        // C s_50_4: const #1s : i
        let s_50_4: i128 = 1;
        // D s_50_5: sub s_50_3 s_50_4
        let s_50_5: i128 = ((s_50_3) - (s_50_4));
        // D s_50_6: read-var N:i
        let s_50_6: i128 = fn_state.N;
        // D s_50_7: call __id(s_50_6)
        let s_50_7: i128 = u__id(state, tracer, s_50_6);
        // C s_50_8: const #8s : i
        let s_50_8: i128 = 8;
        // D s_50_9: mul s_50_8 s_50_7
        let s_50_9: i128 = ((s_50_8) * (s_50_7));
        // D s_50_10: cmp-lt s_50_5 s_50_9
        let s_50_10: bool = ((s_50_5) < (s_50_9));
        // D s_50_11: write-var gs#147210 <= s_50_10
        fn_state.gs_147210 = s_50_10;
        // N s_50_12: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var Bshadow#1158:i
        let s_51_0: i128 = fn_state.Bshadow_1158;
        // D s_51_1: call __id(s_51_0)
        let s_51_1: i128 = u__id(state, tracer, s_51_0);
        // C s_51_2: const #8s : i
        let s_51_2: i128 = 8;
        // D s_51_3: mul s_51_1 s_51_2
        let s_51_3: i128 = ((s_51_1) * (s_51_2));
        // C s_51_4: const #1s : i
        let s_51_4: i128 = 1;
        // D s_51_5: sub s_51_3 s_51_4
        let s_51_5: i128 = ((s_51_3) - (s_51_4));
        // D s_51_6: read-var N:i
        let s_51_6: i128 = fn_state.N;
        // D s_51_7: call __id(s_51_6)
        let s_51_7: i128 = u__id(state, tracer, s_51_6);
        // C s_51_8: const #8s : i
        let s_51_8: i128 = 8;
        // D s_51_9: mul s_51_8 s_51_7
        let s_51_9: i128 = ((s_51_8) * (s_51_7));
        // D s_51_10: cmp-lt s_51_5 s_51_9
        let s_51_10: bool = ((s_51_5) < (s_51_9));
        // D s_51_11: write-var gs#147197 <= s_51_10
        fn_state.gs_147197 = s_51_10;
        // N s_51_12: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_52_0: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call SPESampleMemCopy(s_53_0)
        let s_53_1: () = SPESampleMemCopy(state, tracer, s_53_0);
        // N s_53_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // C s_54_1: const #1u : u8
        let s_54_1: bool = true;
        // D s_54_2: read-var d:i64
        let s_54_2: i64 = fn_state.d;
        // D s_54_3: cast zx s_54_2 -> i
        let s_54_3: i128 = (i128::try_from(s_54_2).unwrap());
        // D s_54_4: read-var s:i64
        let s_54_4: i64 = fn_state.s;
        // D s_54_5: cast zx s_54_4 -> i
        let s_54_5: i128 = (i128::try_from(s_54_4).unwrap());
        // D s_54_6: read-var n:i64
        let s_54_6: i64 = fn_state.n;
        // D s_54_7: cast zx s_54_6 -> i
        let s_54_7: i128 = (i128::try_from(s_54_6).unwrap());
        // D s_54_8: read-var supports_option_a:u8
        let s_54_8: bool = fn_state.supports_option_a;
        // D s_54_9: read-var options_name:u8
        let s_54_9: u8 = fn_state.options_name;
        // D s_54_10: call MismatchedMemCpyException(s_54_8, s_54_3, s_54_5, s_54_7, s_54_0, s_54_1, s_54_9)
        let s_54_10: () = MismatchedMemCpyException(
            state,
            tracer,
            s_54_8,
            s_54_3,
            s_54_5,
            s_54_7,
            s_54_0,
            s_54_1,
            s_54_9,
        );
        // N s_54_11: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var gs#147244 <= s_55_0
        fn_state.gs_147244 = s_55_0;
        // N s_55_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var cpysize:u64
        let s_56_0: u64 = fn_state.cpysize;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 64u16);
        // D s_56_2: read-var postsize:u64
        let s_56_2: u64 = fn_state.postsize;
        // D s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 64u16);
        // D s_56_4: sub s_56_1 s_56_3
        let s_56_4: Bits = ((s_56_1) - (s_56_3));
        // D s_56_5: cast reint s_56_4 -> u64
        let s_56_5: u64 = (s_56_4.value() as u64);
        // D s_56_6: write-var stagecpysize <= s_56_5
        fn_state.stagecpysize = s_56_5;
        // D s_56_7: read-var toaddress:u64
        let s_56_7: u64 = fn_state.toaddress;
        // D s_56_8: read-var fromaddress:u64
        let s_56_8: u64 = fn_state.fromaddress;
        // D s_56_9: read-var cpysize:u64
        let s_56_9: u64 = fn_state.cpysize;
        // D s_56_10: call MemCpyParametersIllformedM(s_56_7, s_56_8, s_56_9)
        let s_56_10: bool = MemCpyParametersIllformedM(
            state,
            tracer,
            s_56_7,
            s_56_8,
            s_56_9,
        );
        // N s_56_11: branch s_56_10 b59 b57
        if s_56_10 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_57_0: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_58_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // C s_59_1: const #0u : u8
        let s_59_1: bool = false;
        // D s_59_2: read-var d:i64
        let s_59_2: i64 = fn_state.d;
        // D s_59_3: cast zx s_59_2 -> i
        let s_59_3: i128 = (i128::try_from(s_59_2).unwrap());
        // D s_59_4: read-var s:i64
        let s_59_4: i64 = fn_state.s;
        // D s_59_5: cast zx s_59_4 -> i
        let s_59_5: i128 = (i128::try_from(s_59_4).unwrap());
        // D s_59_6: read-var n:i64
        let s_59_6: i64 = fn_state.n;
        // D s_59_7: cast zx s_59_6 -> i
        let s_59_7: i128 = (i128::try_from(s_59_6).unwrap());
        // D s_59_8: read-var supports_option_a:u8
        let s_59_8: bool = fn_state.supports_option_a;
        // D s_59_9: read-var options_name:u8
        let s_59_9: u8 = fn_state.options_name;
        // D s_59_10: call MismatchedMemCpyException(s_59_8, s_59_3, s_59_5, s_59_7, s_59_0, s_59_1, s_59_9)
        let s_59_10: () = MismatchedMemCpyException(
            state,
            tracer,
            s_59_8,
            s_59_3,
            s_59_5,
            s_59_7,
            s_59_0,
            s_59_1,
            s_59_9,
        );
        // N s_59_11: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var gs#147242 <= s_60_0
        fn_state.gs_147242 = s_60_0;
        // N s_60_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var supports_option_a:u8
        let s_61_0: bool = fn_state.supports_option_a;
        // N s_61_1: branch s_61_0 b66 b62
        if s_61_0 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #1s : i
        let s_62_0: i128 = 1;
        // D s_62_1: read-var nzcv:u8
        let s_62_1: u8 = fn_state.nzcv;
        // D s_62_2: cast zx s_62_1 -> bv
        let s_62_2: Bits = Bits::new(s_62_1 as u128, 4u16);
        // C s_62_3: const #1u : u64
        let s_62_3: u64 = 1;
        // D s_62_4: bit-extract s_62_2 s_62_0 s_62_3
        let s_62_4: Bits = (Bits::new(
            ((s_62_2) >> (s_62_0)).value(),
            u16::try_from(s_62_3).unwrap(),
        ));
        // D s_62_5: cast reint s_62_4 -> u8
        let s_62_5: bool = ((s_62_4.value()) != 0);
        // C s_62_6: const #0s : i
        let s_62_6: i128 = 0;
        // C s_62_7: const #0u : u64
        let s_62_7: u64 = 0;
        // D s_62_8: cast zx s_62_5 -> u64
        let s_62_8: u64 = (s_62_5 as u64);
        // C s_62_9: const #1u : u64
        let s_62_9: u64 = 1;
        // D s_62_10: and s_62_8 s_62_9
        let s_62_10: u64 = ((s_62_8) & (s_62_9));
        // D s_62_11: cmp-eq s_62_10 s_62_9
        let s_62_11: bool = ((s_62_10) == (s_62_9));
        // D s_62_12: lsl s_62_8 s_62_6
        let s_62_12: u64 = s_62_8 << s_62_6;
        // D s_62_13: or s_62_7 s_62_12
        let s_62_13: u64 = ((s_62_7) | (s_62_12));
        // D s_62_14: cmpl s_62_12
        let s_62_14: u64 = !s_62_12;
        // D s_62_15: and s_62_7 s_62_14
        let s_62_15: u64 = ((s_62_7) & (s_62_14));
        // D s_62_16: select s_62_11 s_62_13 s_62_15
        let s_62_16: u64 = if s_62_11 { s_62_13 } else { s_62_15 };
        // D s_62_17: cast trunc s_62_16 -> u8
        let s_62_17: bool = ((s_62_16) != 0);
        // D s_62_18: cast zx s_62_17 -> bv
        let s_62_18: Bits = Bits::new(s_62_17 as u128, 1u16);
        // C s_62_19: const #0u : u8
        let s_62_19: bool = false;
        // C s_62_20: cast zx s_62_19 -> bv
        let s_62_20: Bits = Bits::new(s_62_19 as u128, 1u16);
        // D s_62_21: cmp-eq s_62_18 s_62_20
        let s_62_21: bool = ((s_62_18) == (s_62_20));
        // N s_62_22: branch s_62_21 b65 b63
        if s_62_21 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_63_0: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_64_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: read-var stage:u32
        let s_65_1: u32 = fn_state.stage;
        // C s_65_2: const #2u : u32
        let s_65_2: u32 = 2;
        // D s_65_3: cmp-eq s_65_1 s_65_2
        let s_65_3: bool = ((s_65_1) == (s_65_2));
        // D s_65_4: read-var d:i64
        let s_65_4: i64 = fn_state.d;
        // D s_65_5: cast zx s_65_4 -> i
        let s_65_5: i128 = (i128::try_from(s_65_4).unwrap());
        // D s_65_6: read-var s:i64
        let s_65_6: i64 = fn_state.s;
        // D s_65_7: cast zx s_65_6 -> i
        let s_65_7: i128 = (i128::try_from(s_65_6).unwrap());
        // D s_65_8: read-var n:i64
        let s_65_8: i64 = fn_state.n;
        // D s_65_9: cast zx s_65_8 -> i
        let s_65_9: i128 = (i128::try_from(s_65_8).unwrap());
        // D s_65_10: read-var supports_option_a:u8
        let s_65_10: bool = fn_state.supports_option_a;
        // D s_65_11: read-var options_name:u8
        let s_65_11: u8 = fn_state.options_name;
        // D s_65_12: call MismatchedMemCpyException(s_65_10, s_65_5, s_65_7, s_65_9, s_65_0, s_65_3, s_65_11)
        let s_65_12: () = MismatchedMemCpyException(
            state,
            tracer,
            s_65_10,
            s_65_5,
            s_65_7,
            s_65_9,
            s_65_0,
            s_65_3,
            s_65_11,
        );
        // N s_65_13: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #1s : i
        let s_66_0: i128 = 1;
        // D s_66_1: read-var nzcv:u8
        let s_66_1: u8 = fn_state.nzcv;
        // D s_66_2: cast zx s_66_1 -> bv
        let s_66_2: Bits = Bits::new(s_66_1 as u128, 4u16);
        // C s_66_3: const #1u : u64
        let s_66_3: u64 = 1;
        // D s_66_4: bit-extract s_66_2 s_66_0 s_66_3
        let s_66_4: Bits = (Bits::new(
            ((s_66_2) >> (s_66_0)).value(),
            u16::try_from(s_66_3).unwrap(),
        ));
        // D s_66_5: cast reint s_66_4 -> u8
        let s_66_5: bool = ((s_66_4.value()) != 0);
        // C s_66_6: const #0s : i
        let s_66_6: i128 = 0;
        // C s_66_7: const #0u : u64
        let s_66_7: u64 = 0;
        // D s_66_8: cast zx s_66_5 -> u64
        let s_66_8: u64 = (s_66_5 as u64);
        // C s_66_9: const #1u : u64
        let s_66_9: u64 = 1;
        // D s_66_10: and s_66_8 s_66_9
        let s_66_10: u64 = ((s_66_8) & (s_66_9));
        // D s_66_11: cmp-eq s_66_10 s_66_9
        let s_66_11: bool = ((s_66_10) == (s_66_9));
        // D s_66_12: lsl s_66_8 s_66_6
        let s_66_12: u64 = s_66_8 << s_66_6;
        // D s_66_13: or s_66_7 s_66_12
        let s_66_13: u64 = ((s_66_7) | (s_66_12));
        // D s_66_14: cmpl s_66_12
        let s_66_14: u64 = !s_66_12;
        // D s_66_15: and s_66_7 s_66_14
        let s_66_15: u64 = ((s_66_7) & (s_66_14));
        // D s_66_16: select s_66_11 s_66_13 s_66_15
        let s_66_16: u64 = if s_66_11 { s_66_13 } else { s_66_15 };
        // D s_66_17: cast trunc s_66_16 -> u8
        let s_66_17: bool = ((s_66_16) != 0);
        // D s_66_18: cast zx s_66_17 -> bv
        let s_66_18: Bits = Bits::new(s_66_17 as u128, 1u16);
        // C s_66_19: const #1u : u8
        let s_66_19: bool = true;
        // C s_66_20: cast zx s_66_19 -> bv
        let s_66_20: Bits = Bits::new(s_66_19 as u128, 1u16);
        // D s_66_21: cmp-eq s_66_18 s_66_20
        let s_66_21: bool = ((s_66_18) == (s_66_20));
        // N s_66_22: branch s_66_21 b69 b67
        if s_66_21 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_67_0: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_68_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #1u : u8
        let s_69_0: bool = true;
        // D s_69_1: read-var stage:u32
        let s_69_1: u32 = fn_state.stage;
        // C s_69_2: const #2u : u32
        let s_69_2: u32 = 2;
        // D s_69_3: cmp-eq s_69_1 s_69_2
        let s_69_3: bool = ((s_69_1) == (s_69_2));
        // D s_69_4: read-var d:i64
        let s_69_4: i64 = fn_state.d;
        // D s_69_5: cast zx s_69_4 -> i
        let s_69_5: i128 = (i128::try_from(s_69_4).unwrap());
        // D s_69_6: read-var s:i64
        let s_69_6: i64 = fn_state.s;
        // D s_69_7: cast zx s_69_6 -> i
        let s_69_7: i128 = (i128::try_from(s_69_6).unwrap());
        // D s_69_8: read-var n:i64
        let s_69_8: i64 = fn_state.n;
        // D s_69_9: cast zx s_69_8 -> i
        let s_69_9: i128 = (i128::try_from(s_69_8).unwrap());
        // D s_69_10: read-var supports_option_a:u8
        let s_69_10: bool = fn_state.supports_option_a;
        // D s_69_11: read-var options_name:u8
        let s_69_11: u8 = fn_state.options_name;
        // D s_69_12: call MismatchedMemCpyException(s_69_10, s_69_5, s_69_7, s_69_9, s_69_0, s_69_3, s_69_11)
        let s_69_12: () = MismatchedMemCpyException(
            state,
            tracer,
            s_69_10,
            s_69_5,
            s_69_7,
            s_69_9,
            s_69_0,
            s_69_3,
            s_69_11,
        );
        // N s_69_13: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #1u : u8
        let s_70_0: bool = true;
        // D s_70_1: write-var gs#147229 <= s_70_0
        fn_state.gs_147229 = s_70_0;
        // N s_70_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #63s : i
        let s_71_0: i128 = 63;
        // D s_71_1: read-var cpysize:u64
        let s_71_1: u64 = fn_state.cpysize;
        // D s_71_2: cast zx s_71_1 -> bv
        let s_71_2: Bits = Bits::new(s_71_1 as u128, 64u16);
        // C s_71_3: const #1u : u64
        let s_71_3: u64 = 1;
        // D s_71_4: bit-extract s_71_2 s_71_0 s_71_3
        let s_71_4: Bits = (Bits::new(
            ((s_71_2) >> (s_71_0)).value(),
            u16::try_from(s_71_3).unwrap(),
        ));
        // D s_71_5: cast reint s_71_4 -> u8
        let s_71_5: bool = ((s_71_4.value()) != 0);
        // C s_71_6: const #0s : i
        let s_71_6: i128 = 0;
        // C s_71_7: const #0u : u64
        let s_71_7: u64 = 0;
        // D s_71_8: cast zx s_71_5 -> u64
        let s_71_8: u64 = (s_71_5 as u64);
        // C s_71_9: const #1u : u64
        let s_71_9: u64 = 1;
        // D s_71_10: and s_71_8 s_71_9
        let s_71_10: u64 = ((s_71_8) & (s_71_9));
        // D s_71_11: cmp-eq s_71_10 s_71_9
        let s_71_11: bool = ((s_71_10) == (s_71_9));
        // D s_71_12: lsl s_71_8 s_71_6
        let s_71_12: u64 = s_71_8 << s_71_6;
        // D s_71_13: or s_71_7 s_71_12
        let s_71_13: u64 = ((s_71_7) | (s_71_12));
        // D s_71_14: cmpl s_71_12
        let s_71_14: u64 = !s_71_12;
        // D s_71_15: and s_71_7 s_71_14
        let s_71_15: u64 = ((s_71_7) & (s_71_14));
        // D s_71_16: select s_71_11 s_71_13 s_71_15
        let s_71_16: u64 = if s_71_11 { s_71_13 } else { s_71_15 };
        // D s_71_17: cast trunc s_71_16 -> u8
        let s_71_17: bool = ((s_71_16) != 0);
        // D s_71_18: cast zx s_71_17 -> bv
        let s_71_18: Bits = Bits::new(s_71_17 as u128, 1u16);
        // C s_71_19: const #1u : u8
        let s_71_19: bool = true;
        // C s_71_20: cast zx s_71_19 -> bv
        let s_71_20: Bits = Bits::new(s_71_19 as u128, 1u16);
        // D s_71_21: cmp-eq s_71_18 s_71_20
        let s_71_21: bool = ((s_71_18) == (s_71_20));
        // N s_71_22: branch s_71_21 b82 b72
        if s_71_21 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_72_0: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var supports_option_a:u8
        let s_73_0: bool = fn_state.supports_option_a;
        // N s_73_1: branch s_73_0 b81 b74
        if s_73_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #2u : u8
        let s_74_0: u8 = 2;
        // D s_74_1: write-var nzcv <= s_74_0
        fn_state.nzcv = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var toaddress:u64
        let s_75_0: u64 = fn_state.toaddress;
        // D s_75_1: read-var fromaddress:u64
        let s_75_1: u64 = fn_state.fromaddress;
        // D s_75_2: read-var cpysize:u64
        let s_75_2: u64 = fn_state.cpysize;
        // D s_75_3: call CPYPreSizeChoice(s_75_0, s_75_1, s_75_2)
        let s_75_3: u64 = CPYPreSizeChoice(state, tracer, s_75_0, s_75_1, s_75_2);
        // D s_75_4: write-var stagecpysize <= s_75_3
        fn_state.stagecpysize = s_75_3;
        // C s_75_5: const #63s : i
        let s_75_5: i128 = 63;
        // D s_75_6: read-var stagecpysize:u64
        let s_75_6: u64 = fn_state.stagecpysize;
        // D s_75_7: cast zx s_75_6 -> bv
        let s_75_7: Bits = Bits::new(s_75_6 as u128, 64u16);
        // C s_75_8: const #1u : u64
        let s_75_8: u64 = 1;
        // D s_75_9: bit-extract s_75_7 s_75_5 s_75_8
        let s_75_9: Bits = (Bits::new(
            ((s_75_7) >> (s_75_5)).value(),
            u16::try_from(s_75_8).unwrap(),
        ));
        // D s_75_10: cast reint s_75_9 -> u8
        let s_75_10: bool = ((s_75_9.value()) != 0);
        // C s_75_11: const #0s : i
        let s_75_11: i128 = 0;
        // C s_75_12: const #0u : u64
        let s_75_12: u64 = 0;
        // D s_75_13: cast zx s_75_10 -> u64
        let s_75_13: u64 = (s_75_10 as u64);
        // C s_75_14: const #1u : u64
        let s_75_14: u64 = 1;
        // D s_75_15: and s_75_13 s_75_14
        let s_75_15: u64 = ((s_75_13) & (s_75_14));
        // D s_75_16: cmp-eq s_75_15 s_75_14
        let s_75_16: bool = ((s_75_15) == (s_75_14));
        // D s_75_17: lsl s_75_13 s_75_11
        let s_75_17: u64 = s_75_13 << s_75_11;
        // D s_75_18: or s_75_12 s_75_17
        let s_75_18: u64 = ((s_75_12) | (s_75_17));
        // D s_75_19: cmpl s_75_17
        let s_75_19: u64 = !s_75_17;
        // D s_75_20: and s_75_12 s_75_19
        let s_75_20: u64 = ((s_75_12) & (s_75_19));
        // D s_75_21: select s_75_16 s_75_18 s_75_20
        let s_75_21: u64 = if s_75_16 { s_75_18 } else { s_75_20 };
        // D s_75_22: cast trunc s_75_21 -> u8
        let s_75_22: bool = ((s_75_21) != 0);
        // C s_75_23: const #63s : i
        let s_75_23: i128 = 63;
        // D s_75_24: read-var cpysize:u64
        let s_75_24: u64 = fn_state.cpysize;
        // D s_75_25: cast zx s_75_24 -> bv
        let s_75_25: Bits = Bits::new(s_75_24 as u128, 64u16);
        // C s_75_26: const #1u : u64
        let s_75_26: u64 = 1;
        // D s_75_27: bit-extract s_75_25 s_75_23 s_75_26
        let s_75_27: Bits = (Bits::new(
            ((s_75_25) >> (s_75_23)).value(),
            u16::try_from(s_75_26).unwrap(),
        ));
        // D s_75_28: cast reint s_75_27 -> u8
        let s_75_28: bool = ((s_75_27.value()) != 0);
        // C s_75_29: const #0s : i
        let s_75_29: i128 = 0;
        // C s_75_30: const #0u : u64
        let s_75_30: u64 = 0;
        // D s_75_31: cast zx s_75_28 -> u64
        let s_75_31: u64 = (s_75_28 as u64);
        // C s_75_32: const #1u : u64
        let s_75_32: u64 = 1;
        // D s_75_33: and s_75_31 s_75_32
        let s_75_33: u64 = ((s_75_31) & (s_75_32));
        // D s_75_34: cmp-eq s_75_33 s_75_32
        let s_75_34: bool = ((s_75_33) == (s_75_32));
        // D s_75_35: lsl s_75_31 s_75_29
        let s_75_35: u64 = s_75_31 << s_75_29;
        // D s_75_36: or s_75_30 s_75_35
        let s_75_36: u64 = ((s_75_30) | (s_75_35));
        // D s_75_37: cmpl s_75_35
        let s_75_37: u64 = !s_75_35;
        // D s_75_38: and s_75_30 s_75_37
        let s_75_38: u64 = ((s_75_30) & (s_75_37));
        // D s_75_39: select s_75_34 s_75_36 s_75_38
        let s_75_39: u64 = if s_75_34 { s_75_36 } else { s_75_38 };
        // D s_75_40: cast trunc s_75_39 -> u8
        let s_75_40: bool = ((s_75_39) != 0);
        // D s_75_41: cast zx s_75_22 -> bv
        let s_75_41: Bits = Bits::new(s_75_22 as u128, 1u16);
        // D s_75_42: cast zx s_75_40 -> bv
        let s_75_42: Bits = Bits::new(s_75_40 as u128, 1u16);
        // D s_75_43: cmp-eq s_75_41 s_75_42
        let s_75_43: bool = ((s_75_41) == (s_75_42));
        // N s_75_44: branch s_75_43 b80 b76
        if s_75_43 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #64s : i
        let s_76_0: i128 = 64;
        // S s_76_1: call Zeros(s_76_0)
        let s_76_1: Bits = Zeros(state, tracer, s_76_0);
        // S s_76_2: cast reint s_76_1 -> u64
        let s_76_2: u64 = (s_76_1.value() as u64);
        // D s_76_3: read-var stagecpysize:u64
        let s_76_3: u64 = fn_state.stagecpysize;
        // D s_76_4: cast zx s_76_3 -> bv
        let s_76_4: Bits = Bits::new(s_76_3 as u128, 64u16);
        // S s_76_5: cast zx s_76_2 -> bv
        let s_76_5: Bits = Bits::new(s_76_2 as u128, 64u16);
        // D s_76_6: cmp-eq s_76_4 s_76_5
        let s_76_6: bool = ((s_76_4) == (s_76_5));
        // D s_76_7: write-var gs#147256 <= s_76_6
        fn_state.gs_147256 = s_76_6;
        // N s_76_8: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#147256:u8
        let s_77_0: bool = fn_state.gs_147256;
        // N s_77_1: assert s_77_0
        let s_77_1: () = assert!(s_77_0);
        // D s_77_2: read-var cpysize:u64
        let s_77_2: u64 = fn_state.cpysize;
        // D s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 64u16);
        // D s_77_4: cast sx s_77_3 -> i
        let s_77_4: i128 = {
            let sign_bit = s_77_3.length() - 1;
            let mut result = s_77_3.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_77_5: cast reint s_77_4 -> i64
        let s_77_5: i64 = (s_77_4 as i64);
        // C s_77_6: const #0s : i
        let s_77_6: i128 = 0;
        // D s_77_7: cast zx s_77_5 -> i
        let s_77_7: i128 = (i128::try_from(s_77_5).unwrap());
        // D s_77_8: cmp-gt s_77_7 s_77_6
        let s_77_8: bool = ((s_77_7) > (s_77_6));
        // N s_77_9: branch s_77_8 b79 b78
        if s_77_8 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var stagecpysize:u64
        let s_78_0: u64 = fn_state.stagecpysize;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 64u16);
        // D s_78_2: cast sx s_78_1 -> i
        let s_78_2: i128 = {
            let sign_bit = s_78_1.length() - 1;
            let mut result = s_78_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_78_3: cast reint s_78_2 -> i64
        let s_78_3: i64 = (s_78_2 as i64);
        // D s_78_4: read-var cpysize:u64
        let s_78_4: u64 = fn_state.cpysize;
        // D s_78_5: cast zx s_78_4 -> bv
        let s_78_5: Bits = Bits::new(s_78_4 as u128, 64u16);
        // D s_78_6: cast sx s_78_5 -> i
        let s_78_6: i128 = {
            let sign_bit = s_78_5.length() - 1;
            let mut result = s_78_5.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_78_7: cast reint s_78_6 -> i64
        let s_78_7: i64 = (s_78_6 as i64);
        // D s_78_8: cast zx s_78_3 -> i
        let s_78_8: i128 = (i128::try_from(s_78_3).unwrap());
        // D s_78_9: cast zx s_78_7 -> i
        let s_78_9: i128 = (i128::try_from(s_78_7).unwrap());
        // D s_78_10: cmp-ge s_78_8 s_78_9
        let s_78_10: bool = ((s_78_8) >= (s_78_9));
        // N s_78_11: assert s_78_10
        let s_78_11: () = assert!(s_78_10);
        // N s_78_12: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var stagecpysize:u64
        let s_79_0: u64 = fn_state.stagecpysize;
        // D s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 64u16);
        // D s_79_2: cast sx s_79_1 -> i
        let s_79_2: i128 = {
            let sign_bit = s_79_1.length() - 1;
            let mut result = s_79_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_79_3: cast reint s_79_2 -> i64
        let s_79_3: i64 = (s_79_2 as i64);
        // D s_79_4: read-var cpysize:u64
        let s_79_4: u64 = fn_state.cpysize;
        // D s_79_5: cast zx s_79_4 -> bv
        let s_79_5: Bits = Bits::new(s_79_4 as u128, 64u16);
        // D s_79_6: cast sx s_79_5 -> i
        let s_79_6: i128 = {
            let sign_bit = s_79_5.length() - 1;
            let mut result = s_79_5.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_79_7: cast reint s_79_6 -> i64
        let s_79_7: i64 = (s_79_6 as i64);
        // D s_79_8: cast zx s_79_3 -> i
        let s_79_8: i128 = (i128::try_from(s_79_3).unwrap());
        // D s_79_9: cast zx s_79_7 -> i
        let s_79_9: i128 = (i128::try_from(s_79_7).unwrap());
        // D s_79_10: cmp-le s_79_8 s_79_9
        let s_79_10: bool = ((s_79_8) <= (s_79_9));
        // N s_79_11: assert s_79_10
        let s_79_11: () = assert!(s_79_10);
        // N s_79_12: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #1u : u8
        let s_80_0: bool = true;
        // D s_80_1: write-var gs#147256 <= s_80_0
        fn_state.gs_147256 = s_80_0;
        // N s_80_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #0u : u8
        let s_81_0: u8 = 0;
        // D s_81_1: write-var nzcv <= s_81_0
        fn_state.nzcv = s_81_0;
        // D s_81_2: read-var toaddress:u64
        let s_81_2: u64 = fn_state.toaddress;
        // D s_81_3: cast zx s_81_2 -> bv
        let s_81_3: Bits = Bits::new(s_81_2 as u128, 64u16);
        // D s_81_4: read-var cpysize:u64
        let s_81_4: u64 = fn_state.cpysize;
        // D s_81_5: cast zx s_81_4 -> bv
        let s_81_5: Bits = Bits::new(s_81_4 as u128, 64u16);
        // D s_81_6: add s_81_3 s_81_5
        let s_81_6: Bits = (s_81_3 + s_81_5);
        // D s_81_7: cast reint s_81_6 -> u64
        let s_81_7: u64 = (s_81_6.value() as u64);
        // D s_81_8: write-var toaddress <= s_81_7
        fn_state.toaddress = s_81_7;
        // D s_81_9: read-var fromaddress:u64
        let s_81_9: u64 = fn_state.fromaddress;
        // D s_81_10: cast zx s_81_9 -> bv
        let s_81_10: Bits = Bits::new(s_81_9 as u128, 64u16);
        // D s_81_11: read-var cpysize:u64
        let s_81_11: u64 = fn_state.cpysize;
        // D s_81_12: cast zx s_81_11 -> bv
        let s_81_12: Bits = Bits::new(s_81_11 as u128, 64u16);
        // D s_81_13: add s_81_10 s_81_12
        let s_81_13: Bits = (s_81_10 + s_81_12);
        // D s_81_14: cast reint s_81_13 -> u64
        let s_81_14: u64 = (s_81_13.value() as u64);
        // D s_81_15: write-var fromaddress <= s_81_14
        fn_state.fromaddress = s_81_14;
        // C s_81_16: const #64s : i
        let s_81_16: i128 = 64;
        // S s_81_17: call Zeros(s_81_16)
        let s_81_17: Bits = Zeros(state, tracer, s_81_16);
        // S s_81_18: cast reint s_81_17 -> u64
        let s_81_18: u64 = (s_81_17.value() as u64);
        // S s_81_19: cast zx s_81_18 -> bv
        let s_81_19: Bits = Bits::new(s_81_18 as u128, 64u16);
        // D s_81_20: read-var cpysize:u64
        let s_81_20: u64 = fn_state.cpysize;
        // D s_81_21: cast zx s_81_20 -> bv
        let s_81_21: Bits = Bits::new(s_81_20 as u128, 64u16);
        // D s_81_22: sub s_81_19 s_81_21
        let s_81_22: Bits = ((s_81_19) - (s_81_21));
        // D s_81_23: cast reint s_81_22 -> u64
        let s_81_23: u64 = (s_81_22.value() as u64);
        // D s_81_24: write-var cpysize <= s_81_23
        fn_state.cpysize = s_81_23;
        // N s_81_25: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #9223372036854775807u : u64
        let s_82_0: u64 = 9223372036854775807;
        // D s_82_1: write-var cpysize <= s_82_0
        fn_state.cpysize = s_82_0;
        // N s_82_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call AArch64_IsUnprivAccessPriv(s_83_0)
        let s_83_1: bool = AArch64_IsUnprivAccessPriv(state, tracer, s_83_0);
        // D s_83_2: write-var wprivileged <= s_83_1
        fn_state.wprivileged = s_83_1;
        // N s_83_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #() : ()
        let s_84_0: () = ();
        // S s_84_1: call AArch64_IsUnprivAccessPriv(s_84_0)
        let s_84_1: bool = AArch64_IsUnprivAccessPriv(state, tracer, s_84_0);
        // D s_84_2: write-var rprivileged <= s_84_1
        fn_state.rprivileged = s_84_1;
        // N s_84_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
