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
use CPYOptionA::*;
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
use AArch64_IsUnprivAccessPriv::*;
use SPESampleMemCopy::*;
use MemCpyDirectionChoice::*;
use CPYPreSizeChoice::*;
use common::*;
pub fn execute_aarch64_instrs_memory_mcpymset_cpy<T: Tracer>(
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
        gs_146585: bool,
        raccdesc: ProductType9878976b5bcce9c9,
        ga_251505: i128,
        gs_146609: bool,
        toaddress: u64,
        gs_146530: bool,
        ga_251531: Bits,
        ga_251530: i128,
        ga_251548: Bits,
        rprivileged: bool,
        fromaddress: u64,
        ga_251506: Bits,
        gs_146549: bool,
        waccdesc: ProductType9878976b5bcce9c9,
        gs_146631: bool,
        ga_251485: i128,
        readdata: Bits,
        Bshadow_1139: i128,
        gs_146496: bool,
        Bshadow_1138: i128,
        B: i128,
        cpysize: u64,
        ga_251547: i128,
        postsize: u64,
        gs_146603: bool,
        forward: bool,
        gs_146479: bool,
        nzcv: u8,
        gs_146570: bool,
        wprivileged: bool,
        supports_option_a: bool,
        gs_146583: bool,
        N: i128,
        ga_251486: Bits,
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
        // S s_0_64: call CPYOptionA(s_0_63)
        let s_0_64: bool = CPYOptionA(state, tracer, s_0_63);
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
        // N s_0_88: branch s_0_87 b109 b1
        if s_0_87 {
            return block_109(state, tracer, fn_state);
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
        // N s_2_22: branch s_2_21 b108 b3
        if s_2_21 {
            return block_108(state, tracer, fn_state);
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
        // N s_4_55: branch s_4_54 b80 b5
        if s_4_54 {
            return block_80(state, tracer, fn_state);
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
        // N s_5_2: branch s_5_1 b79 b6
        if s_5_1 {
            return block_79(state, tracer, fn_state);
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
        // D s_6_7: write-var gs#146570 <= s_6_6
        fn_state.gs_146570 = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#146570:u8
        let s_7_0: bool = fn_state.gs_146570;
        // N s_7_1: branch s_7_0 b70 b8
        if s_7_0 {
            return block_70(state, tracer, fn_state);
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
        // N s_9_44: branch s_9_43 b69 b10
        if s_9_43 {
            return block_69(state, tracer, fn_state);
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
        // D s_10_7: write-var gs#146583 <= s_10_6
        fn_state.gs_146583 = s_10_6;
        // N s_10_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#146583:u8
        let s_11_0: bool = fn_state.gs_146583;
        // N s_11_1: assert s_11_0
        let s_11_1: () = assert!(s_11_0);
        // D s_11_2: read-var stage:u32
        let s_11_2: u32 = fn_state.stage;
        // C s_11_3: const #1u : u32
        let s_11_3: u32 = 1;
        // D s_11_4: cmp-eq s_11_2 s_11_3
        let s_11_4: bool = ((s_11_2) == (s_11_3));
        // N s_11_5: branch s_11_4 b65 b12
        if s_11_4 {
            return block_65(state, tracer, fn_state);
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
        // N s_12_7: branch s_12_6 b64 b13
        if s_12_6 {
            return block_64(state, tracer, fn_state);
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
        // D s_13_4: write-var gs#146585 <= s_13_3
        fn_state.gs_146585 = s_13_3;
        // N s_13_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#146585:u8
        let s_14_0: bool = fn_state.gs_146585;
        // N s_14_1: branch s_14_0 b63 b15
        if s_14_0 {
            return block_63(state, tracer, fn_state);
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
        // N s_17_2: branch s_17_1 b62 b18
        if s_17_1 {
            return block_62(state, tracer, fn_state);
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
        // N s_19_1: branch s_19_0 b43 b20
        if s_19_0 {
            return block_43(state, tracer, fn_state);
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
        // N s_21_6: branch s_21_5 b39 b22
        if s_21_5 {
            return block_39(state, tracer, fn_state);
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
        // D s_22_12: write-var Bshadow#1138 <= s_22_11
        fn_state.Bshadow_1138 = s_22_11;
        // C s_22_13: const #3s : i
        let s_22_13: i128 = 3;
        // D s_22_14: read-var nzcv:u8
        let s_22_14: u8 = fn_state.nzcv;
        // D s_22_15: cast zx s_22_14 -> bv
        let s_22_15: Bits = Bits::new(s_22_14 as u128, 4u16);
        // C s_22_16: const #1u : u64
        let s_22_16: u64 = 1;
        // D s_22_17: bit-extract s_22_15 s_22_13 s_22_16
        let s_22_17: Bits = (Bits::new(
            ((s_22_15) >> (s_22_13)).value(),
            u16::try_from(s_22_16).unwrap(),
        ));
        // D s_22_18: cast reint s_22_17 -> u8
        let s_22_18: bool = ((s_22_17.value()) != 0);
        // C s_22_19: const #0s : i
        let s_22_19: i128 = 0;
        // C s_22_20: const #0u : u64
        let s_22_20: u64 = 0;
        // D s_22_21: cast zx s_22_18 -> u64
        let s_22_21: u64 = (s_22_18 as u64);
        // C s_22_22: const #1u : u64
        let s_22_22: u64 = 1;
        // D s_22_23: and s_22_21 s_22_22
        let s_22_23: u64 = ((s_22_21) & (s_22_22));
        // D s_22_24: cmp-eq s_22_23 s_22_22
        let s_22_24: bool = ((s_22_23) == (s_22_22));
        // D s_22_25: lsl s_22_21 s_22_19
        let s_22_25: u64 = s_22_21 << s_22_19;
        // D s_22_26: or s_22_20 s_22_25
        let s_22_26: u64 = ((s_22_20) | (s_22_25));
        // D s_22_27: cmpl s_22_25
        let s_22_27: u64 = !s_22_25;
        // D s_22_28: and s_22_20 s_22_27
        let s_22_28: u64 = ((s_22_20) & (s_22_27));
        // D s_22_29: select s_22_24 s_22_26 s_22_28
        let s_22_29: u64 = if s_22_24 { s_22_26 } else { s_22_28 };
        // D s_22_30: cast trunc s_22_29 -> u8
        let s_22_30: bool = ((s_22_29) != 0);
        // D s_22_31: cast zx s_22_30 -> bv
        let s_22_31: Bits = Bits::new(s_22_30 as u128, 1u16);
        // C s_22_32: const #0u : u8
        let s_22_32: bool = false;
        // C s_22_33: cast zx s_22_32 -> bv
        let s_22_33: Bits = Bits::new(s_22_32 as u128, 1u16);
        // D s_22_34: cmp-eq s_22_31 s_22_33
        let s_22_34: bool = ((s_22_31) == (s_22_33));
        // N s_22_35: branch s_22_34 b33 b23
        if s_22_34 {
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
        // D s_23_0: read-var Bshadow#1138:i
        let s_23_0: i128 = fn_state.Bshadow_1138;
        // D s_23_1: call __id(s_23_0)
        let s_23_1: i128 = u__id(state, tracer, s_23_0);
        // C s_23_2: const #8s : i
        let s_23_2: i128 = 8;
        // D s_23_3: mul s_23_1 s_23_2
        let s_23_3: i128 = ((s_23_1) * (s_23_2));
        // C s_23_4: const #1s : i
        let s_23_4: i128 = 1;
        // D s_23_5: sub s_23_3 s_23_4
        let s_23_5: i128 = ((s_23_3) - (s_23_4));
        // C s_23_6: const #0s : i
        let s_23_6: i128 = 0;
        // D s_23_7: cmp-le s_23_6 s_23_5
        let s_23_7: bool = ((s_23_6) <= (s_23_5));
        // N s_23_8: branch s_23_7 b32 b24
        if s_23_7 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#146479 <= s_24_0
        fn_state.gs_146479 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#146479:u8
        let s_25_0: bool = fn_state.gs_146479;
        // N s_25_1: assert s_25_0
        let s_25_1: () = assert!(s_25_0);
        // C s_25_2: const #8s : i
        let s_25_2: i128 = 8;
        // D s_25_3: read-var Bshadow#1138:i
        let s_25_3: i128 = fn_state.Bshadow_1138;
        // D s_25_4: mul s_25_3 s_25_2
        let s_25_4: i128 = ((s_25_3) * (s_25_2));
        // C s_25_5: const #1s : i
        let s_25_5: i128 = 1;
        // D s_25_6: sub s_25_4 s_25_5
        let s_25_6: i128 = ((s_25_4) - (s_25_5));
        // D s_25_7: write-var ga#251547 <= s_25_6
        fn_state.ga_251547 = s_25_6;
        // D s_25_8: read-var fromaddress:u64
        let s_25_8: u64 = fn_state.fromaddress;
        // D s_25_9: cast zx s_25_8 -> bv
        let s_25_9: Bits = Bits::new(s_25_8 as u128, 64u16);
        // D s_25_10: read-var Bshadow#1138:i
        let s_25_10: i128 = fn_state.Bshadow_1138;
        // D s_25_11: cast cvt s_25_10 -> bv
        let s_25_11: Bits = Bits::new(s_25_10 as u128, 128);
        // D s_25_12: sub s_25_9 s_25_11
        let s_25_12: Bits = ((s_25_9) - (s_25_11));
        // D s_25_13: cast reint s_25_12 -> u64
        let s_25_13: u64 = (s_25_12.value() as u64);
        // D s_25_14: read-var Bshadow#1138:i
        let s_25_14: i128 = fn_state.Bshadow_1138;
        // D s_25_15: read-var raccdesc:struct
        let s_25_15: ProductType9878976b5bcce9c9 = fn_state.raccdesc;
        // D s_25_16: call Mem_read(s_25_13, s_25_14, s_25_15)
        let s_25_16: Bits = Mem_read(state, tracer, s_25_13, s_25_14, s_25_15);
        // D s_25_17: write-var ga#251548 <= s_25_16
        fn_state.ga_251548 = s_25_16;
        // N s_25_18: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0s : i
        let s_26_0: i128 = 0;
        // D s_26_1: read-var readdata:bv
        let s_26_1: Bits = fn_state.readdata;
        // D s_26_2: read-var ga#251547:i
        let s_26_2: i128 = fn_state.ga_251547;
        // D s_26_3: read-var ga#251548:bv
        let s_26_3: Bits = fn_state.ga_251548;
        // D s_26_4: sub s_26_2 s_26_0
        let s_26_4: i128 = ((s_26_2) - (s_26_0));
        // C s_26_5: const #1u : u64
        let s_26_5: u64 = 1;
        // C s_26_6: cast zx s_26_5 -> bv
        let s_26_6: Bits = Bits::new(s_26_5 as u128, 64u16);
        // D s_26_7: lsl s_26_6 s_26_4
        let s_26_7: Bits = s_26_6 << s_26_4;
        // D s_26_8: sub s_26_7 s_26_6
        let s_26_8: Bits = ((s_26_7) - (s_26_6));
        // D s_26_9: and s_26_3 s_26_8
        let s_26_9: Bits = ((s_26_3) & (s_26_8));
        // D s_26_10: lsl s_26_9 s_26_0
        let s_26_10: Bits = s_26_9 << s_26_0;
        // D s_26_11: lsl s_26_8 s_26_0
        let s_26_11: Bits = s_26_8 << s_26_0;
        // D s_26_12: cmpl s_26_11
        let s_26_12: Bits = !s_26_11;
        // D s_26_13: and s_26_1 s_26_12
        let s_26_13: Bits = ((s_26_1) & (s_26_12));
        // D s_26_14: or s_26_13 s_26_10
        let s_26_14: Bits = ((s_26_13) | (s_26_10));
        // D s_26_15: write-var readdata <= s_26_14
        fn_state.readdata = s_26_14;
        // D s_26_16: read-var toaddress:u64
        let s_26_16: u64 = fn_state.toaddress;
        // D s_26_17: cast zx s_26_16 -> bv
        let s_26_17: Bits = Bits::new(s_26_16 as u128, 64u16);
        // D s_26_18: read-var Bshadow#1138:i
        let s_26_18: i128 = fn_state.Bshadow_1138;
        // D s_26_19: cast cvt s_26_18 -> bv
        let s_26_19: Bits = Bits::new(s_26_18 as u128, 128);
        // D s_26_20: sub s_26_17 s_26_19
        let s_26_20: Bits = ((s_26_17) - (s_26_19));
        // D s_26_21: cast reint s_26_20 -> u64
        let s_26_21: u64 = (s_26_20.value() as u64);
        // C s_26_22: const #8s : i
        let s_26_22: i128 = 8;
        // D s_26_23: read-var Bshadow#1138:i
        let s_26_23: i128 = fn_state.Bshadow_1138;
        // D s_26_24: mul s_26_23 s_26_22
        let s_26_24: i128 = ((s_26_23) * (s_26_22));
        // C s_26_25: const #1s : i
        let s_26_25: i128 = 1;
        // D s_26_26: sub s_26_24 s_26_25
        let s_26_26: i128 = ((s_26_24) - (s_26_25));
        // C s_26_27: const #0s : i
        let s_26_27: i128 = 0;
        // D s_26_28: read-var readdata:bv
        let s_26_28: Bits = fn_state.readdata;
        // C s_26_29: const #1s : i64
        let s_26_29: i64 = 1;
        // C s_26_30: cast zx s_26_29 -> i
        let s_26_30: i128 = (i128::try_from(s_26_29).unwrap());
        // D s_26_31: sub s_26_26 s_26_27
        let s_26_31: i128 = ((s_26_26) - (s_26_27));
        // D s_26_32: add s_26_31 s_26_30
        let s_26_32: i128 = (s_26_31 + s_26_30);
        // D s_26_33: bit-extract s_26_28 s_26_27 s_26_32
        let s_26_33: Bits = (Bits::new(
            ((s_26_28) >> (s_26_27)).value(),
            u16::try_from(s_26_32).unwrap(),
        ));
        // D s_26_34: read-var Bshadow#1138:i
        let s_26_34: i128 = fn_state.Bshadow_1138;
        // D s_26_35: read-var waccdesc:struct
        let s_26_35: ProductType9878976b5bcce9c9 = fn_state.waccdesc;
        // D s_26_36: call Mem_set(s_26_21, s_26_34, s_26_35, s_26_33)
        let s_26_36: () = Mem_set(state, tracer, s_26_21, s_26_34, s_26_35, s_26_33);
        // N s_26_37: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var fromaddress:u64
        let s_27_0: u64 = fn_state.fromaddress;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 64u16);
        // D s_27_2: read-var Bshadow#1138:i
        let s_27_2: i128 = fn_state.Bshadow_1138;
        // D s_27_3: cast cvt s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 128);
        // D s_27_4: sub s_27_1 s_27_3
        let s_27_4: Bits = ((s_27_1) - (s_27_3));
        // D s_27_5: cast reint s_27_4 -> u64
        let s_27_5: u64 = (s_27_4.value() as u64);
        // D s_27_6: write-var fromaddress <= s_27_5
        fn_state.fromaddress = s_27_5;
        // D s_27_7: read-var toaddress:u64
        let s_27_7: u64 = fn_state.toaddress;
        // D s_27_8: cast zx s_27_7 -> bv
        let s_27_8: Bits = Bits::new(s_27_7 as u128, 64u16);
        // D s_27_9: read-var Bshadow#1138:i
        let s_27_9: i128 = fn_state.Bshadow_1138;
        // D s_27_10: cast cvt s_27_9 -> bv
        let s_27_10: Bits = Bits::new(s_27_9 as u128, 128);
        // D s_27_11: sub s_27_8 s_27_10
        let s_27_11: Bits = ((s_27_8) - (s_27_10));
        // D s_27_12: cast reint s_27_11 -> u64
        let s_27_12: u64 = (s_27_11.value() as u64);
        // D s_27_13: write-var toaddress <= s_27_12
        fn_state.toaddress = s_27_12;
        // N s_27_14: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var cpysize:u64
        let s_28_0: u64 = fn_state.cpysize;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 64u16);
        // D s_28_2: read-var B:i
        let s_28_2: i128 = fn_state.B;
        // D s_28_3: cast cvt s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 128);
        // D s_28_4: sub s_28_1 s_28_3
        let s_28_4: Bits = ((s_28_1) - (s_28_3));
        // D s_28_5: cast reint s_28_4 -> u64
        let s_28_5: u64 = (s_28_4.value() as u64);
        // D s_28_6: write-var cpysize <= s_28_5
        fn_state.cpysize = s_28_5;
        // D s_28_7: read-var stagecpysize:u64
        let s_28_7: u64 = fn_state.stagecpysize;
        // D s_28_8: cast zx s_28_7 -> bv
        let s_28_8: Bits = Bits::new(s_28_7 as u128, 64u16);
        // D s_28_9: read-var B:i
        let s_28_9: i128 = fn_state.B;
        // D s_28_10: cast cvt s_28_9 -> bv
        let s_28_10: Bits = Bits::new(s_28_9 as u128, 128);
        // D s_28_11: sub s_28_8 s_28_10
        let s_28_11: Bits = ((s_28_8) - (s_28_10));
        // D s_28_12: cast reint s_28_11 -> u64
        let s_28_12: u64 = (s_28_11.value() as u64);
        // D s_28_13: write-var stagecpysize <= s_28_12
        fn_state.stagecpysize = s_28_12;
        // D s_28_14: read-var stage:u32
        let s_28_14: u32 = fn_state.stage;
        // C s_28_15: const #0u : u32
        let s_28_15: u32 = 0;
        // D s_28_16: cmp-eq s_28_14 s_28_15
        let s_28_16: bool = ((s_28_14) == (s_28_15));
        // N s_28_17: branch s_28_16 b31 b29
        if s_28_16 {
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
        // D s_32_0: read-var Bshadow#1138:i
        let s_32_0: i128 = fn_state.Bshadow_1138;
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
        // D s_32_11: write-var gs#146479 <= s_32_10
        fn_state.gs_146479 = s_32_10;
        // N s_32_12: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var Bshadow#1138:i
        let s_33_0: i128 = fn_state.Bshadow_1138;
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
        // C s_33_6: const #0s : i
        let s_33_6: i128 = 0;
        // D s_33_7: cmp-le s_33_6 s_33_5
        let s_33_7: bool = ((s_33_6) <= (s_33_5));
        // N s_33_8: branch s_33_7 b38 b34
        if s_33_7 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#146496 <= s_34_0
        fn_state.gs_146496 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#146496:u8
        let s_35_0: bool = fn_state.gs_146496;
        // N s_35_1: assert s_35_0
        let s_35_1: () = assert!(s_35_0);
        // C s_35_2: const #8s : i
        let s_35_2: i128 = 8;
        // D s_35_3: read-var Bshadow#1138:i
        let s_35_3: i128 = fn_state.Bshadow_1138;
        // D s_35_4: mul s_35_3 s_35_2
        let s_35_4: i128 = ((s_35_3) * (s_35_2));
        // C s_35_5: const #1s : i
        let s_35_5: i128 = 1;
        // D s_35_6: sub s_35_4 s_35_5
        let s_35_6: i128 = ((s_35_4) - (s_35_5));
        // D s_35_7: write-var ga#251530 <= s_35_6
        fn_state.ga_251530 = s_35_6;
        // D s_35_8: read-var fromaddress:u64
        let s_35_8: u64 = fn_state.fromaddress;
        // D s_35_9: read-var Bshadow#1138:i
        let s_35_9: i128 = fn_state.Bshadow_1138;
        // D s_35_10: read-var raccdesc:struct
        let s_35_10: ProductType9878976b5bcce9c9 = fn_state.raccdesc;
        // D s_35_11: call Mem_read(s_35_8, s_35_9, s_35_10)
        let s_35_11: Bits = Mem_read(state, tracer, s_35_8, s_35_9, s_35_10);
        // D s_35_12: write-var ga#251531 <= s_35_11
        fn_state.ga_251531 = s_35_11;
        // N s_35_13: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #0s : i
        let s_36_0: i128 = 0;
        // D s_36_1: read-var readdata:bv
        let s_36_1: Bits = fn_state.readdata;
        // D s_36_2: read-var ga#251530:i
        let s_36_2: i128 = fn_state.ga_251530;
        // D s_36_3: read-var ga#251531:bv
        let s_36_3: Bits = fn_state.ga_251531;
        // D s_36_4: sub s_36_2 s_36_0
        let s_36_4: i128 = ((s_36_2) - (s_36_0));
        // C s_36_5: const #1u : u64
        let s_36_5: u64 = 1;
        // C s_36_6: cast zx s_36_5 -> bv
        let s_36_6: Bits = Bits::new(s_36_5 as u128, 64u16);
        // D s_36_7: lsl s_36_6 s_36_4
        let s_36_7: Bits = s_36_6 << s_36_4;
        // D s_36_8: sub s_36_7 s_36_6
        let s_36_8: Bits = ((s_36_7) - (s_36_6));
        // D s_36_9: and s_36_3 s_36_8
        let s_36_9: Bits = ((s_36_3) & (s_36_8));
        // D s_36_10: lsl s_36_9 s_36_0
        let s_36_10: Bits = s_36_9 << s_36_0;
        // D s_36_11: lsl s_36_8 s_36_0
        let s_36_11: Bits = s_36_8 << s_36_0;
        // D s_36_12: cmpl s_36_11
        let s_36_12: Bits = !s_36_11;
        // D s_36_13: and s_36_1 s_36_12
        let s_36_13: Bits = ((s_36_1) & (s_36_12));
        // D s_36_14: or s_36_13 s_36_10
        let s_36_14: Bits = ((s_36_13) | (s_36_10));
        // D s_36_15: write-var readdata <= s_36_14
        fn_state.readdata = s_36_14;
        // C s_36_16: const #8s : i
        let s_36_16: i128 = 8;
        // D s_36_17: read-var Bshadow#1138:i
        let s_36_17: i128 = fn_state.Bshadow_1138;
        // D s_36_18: mul s_36_17 s_36_16
        let s_36_18: i128 = ((s_36_17) * (s_36_16));
        // C s_36_19: const #1s : i
        let s_36_19: i128 = 1;
        // D s_36_20: sub s_36_18 s_36_19
        let s_36_20: i128 = ((s_36_18) - (s_36_19));
        // C s_36_21: const #0s : i
        let s_36_21: i128 = 0;
        // D s_36_22: read-var readdata:bv
        let s_36_22: Bits = fn_state.readdata;
        // C s_36_23: const #1s : i64
        let s_36_23: i64 = 1;
        // C s_36_24: cast zx s_36_23 -> i
        let s_36_24: i128 = (i128::try_from(s_36_23).unwrap());
        // D s_36_25: sub s_36_20 s_36_21
        let s_36_25: i128 = ((s_36_20) - (s_36_21));
        // D s_36_26: add s_36_25 s_36_24
        let s_36_26: i128 = (s_36_25 + s_36_24);
        // D s_36_27: bit-extract s_36_22 s_36_21 s_36_26
        let s_36_27: Bits = (Bits::new(
            ((s_36_22) >> (s_36_21)).value(),
            u16::try_from(s_36_26).unwrap(),
        ));
        // D s_36_28: read-var toaddress:u64
        let s_36_28: u64 = fn_state.toaddress;
        // D s_36_29: read-var Bshadow#1138:i
        let s_36_29: i128 = fn_state.Bshadow_1138;
        // D s_36_30: read-var waccdesc:struct
        let s_36_30: ProductType9878976b5bcce9c9 = fn_state.waccdesc;
        // D s_36_31: call Mem_set(s_36_28, s_36_29, s_36_30, s_36_27)
        let s_36_31: () = Mem_set(state, tracer, s_36_28, s_36_29, s_36_30, s_36_27);
        // N s_36_32: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var fromaddress:u64
        let s_37_0: u64 = fn_state.fromaddress;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 64u16);
        // D s_37_2: read-var Bshadow#1138:i
        let s_37_2: i128 = fn_state.Bshadow_1138;
        // D s_37_3: cast cvt s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 128);
        // D s_37_4: add s_37_1 s_37_3
        let s_37_4: Bits = (s_37_1 + s_37_3);
        // D s_37_5: cast reint s_37_4 -> u64
        let s_37_5: u64 = (s_37_4.value() as u64);
        // D s_37_6: write-var fromaddress <= s_37_5
        fn_state.fromaddress = s_37_5;
        // D s_37_7: read-var toaddress:u64
        let s_37_7: u64 = fn_state.toaddress;
        // D s_37_8: cast zx s_37_7 -> bv
        let s_37_8: Bits = Bits::new(s_37_7 as u128, 64u16);
        // D s_37_9: read-var Bshadow#1138:i
        let s_37_9: i128 = fn_state.Bshadow_1138;
        // D s_37_10: cast cvt s_37_9 -> bv
        let s_37_10: Bits = Bits::new(s_37_9 as u128, 128);
        // D s_37_11: add s_37_8 s_37_10
        let s_37_11: Bits = (s_37_8 + s_37_10);
        // D s_37_12: cast reint s_37_11 -> u64
        let s_37_12: u64 = (s_37_11.value() as u64);
        // D s_37_13: write-var toaddress <= s_37_12
        fn_state.toaddress = s_37_12;
        // N s_37_14: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var Bshadow#1138:i
        let s_38_0: i128 = fn_state.Bshadow_1138;
        // D s_38_1: call __id(s_38_0)
        let s_38_1: i128 = u__id(state, tracer, s_38_0);
        // C s_38_2: const #8s : i
        let s_38_2: i128 = 8;
        // D s_38_3: mul s_38_1 s_38_2
        let s_38_3: i128 = ((s_38_1) * (s_38_2));
        // C s_38_4: const #1s : i
        let s_38_4: i128 = 1;
        // D s_38_5: sub s_38_3 s_38_4
        let s_38_5: i128 = ((s_38_3) - (s_38_4));
        // D s_38_6: read-var N:i
        let s_38_6: i128 = fn_state.N;
        // D s_38_7: call __id(s_38_6)
        let s_38_7: i128 = u__id(state, tracer, s_38_6);
        // C s_38_8: const #8s : i
        let s_38_8: i128 = 8;
        // D s_38_9: mul s_38_8 s_38_7
        let s_38_9: i128 = ((s_38_8) * (s_38_7));
        // D s_38_10: cmp-lt s_38_5 s_38_9
        let s_38_10: bool = ((s_38_5) < (s_38_9));
        // D s_38_11: write-var gs#146496 <= s_38_10
        fn_state.gs_146496 = s_38_10;
        // N s_38_12: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_39_0: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var stage:u32
        let s_40_0: u32 = fn_state.stage;
        // C s_40_1: const #0u : u32
        let s_40_1: u32 = 0;
        // D s_40_2: cmp-eq s_40_0 s_40_1
        let s_40_2: bool = ((s_40_0) == (s_40_1));
        // N s_40_3: branch s_40_2 b42 b41
        if s_40_2 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_41_0: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #64s : i64
        let s_42_0: i64 = 64;
        // D s_42_1: read-var n:i64
        let s_42_1: i64 = fn_state.n;
        // D s_42_2: cast zx s_42_1 -> i
        let s_42_2: i128 = (i128::try_from(s_42_1).unwrap());
        // D s_42_3: read-var cpysize:u64
        let s_42_3: u64 = fn_state.cpysize;
        // D s_42_4: cast zx s_42_3 -> bv
        let s_42_4: Bits = Bits::new(s_42_3 as u128, 64u16);
        // D s_42_5: call X_set(s_42_2, s_42_0, s_42_4)
        let s_42_5: () = X_set(state, tracer, s_42_2, s_42_0, s_42_4);
        // C s_42_6: const #64s : i64
        let s_42_6: i64 = 64;
        // D s_42_7: read-var d:i64
        let s_42_7: i64 = fn_state.d;
        // D s_42_8: cast zx s_42_7 -> i
        let s_42_8: i128 = (i128::try_from(s_42_7).unwrap());
        // D s_42_9: read-var toaddress:u64
        let s_42_9: u64 = fn_state.toaddress;
        // D s_42_10: cast zx s_42_9 -> bv
        let s_42_10: Bits = Bits::new(s_42_9 as u128, 64u16);
        // D s_42_11: call X_set(s_42_8, s_42_6, s_42_10)
        let s_42_11: () = X_set(state, tracer, s_42_8, s_42_6, s_42_10);
        // C s_42_12: const #64s : i64
        let s_42_12: i64 = 64;
        // D s_42_13: read-var s:i64
        let s_42_13: i64 = fn_state.s;
        // D s_42_14: cast zx s_42_13 -> i
        let s_42_14: i128 = (i128::try_from(s_42_13).unwrap());
        // D s_42_15: read-var fromaddress:u64
        let s_42_15: u64 = fn_state.fromaddress;
        // D s_42_16: cast zx s_42_15 -> bv
        let s_42_16: Bits = Bits::new(s_42_15 as u128, 64u16);
        // D s_42_17: call X_set(s_42_14, s_42_12, s_42_16)
        let s_42_17: () = X_set(state, tracer, s_42_14, s_42_12, s_42_16);
        // C s_42_18: const #3s : i
        let s_42_18: i128 = 3;
        // D s_42_19: read-var nzcv:u8
        let s_42_19: u8 = fn_state.nzcv;
        // D s_42_20: cast zx s_42_19 -> bv
        let s_42_20: Bits = Bits::new(s_42_19 as u128, 4u16);
        // C s_42_21: const #1s : i64
        let s_42_21: i64 = 1;
        // C s_42_22: cast zx s_42_21 -> i
        let s_42_22: i128 = (i128::try_from(s_42_21).unwrap());
        // C s_42_23: const #0s : i
        let s_42_23: i128 = 0;
        // C s_42_24: add s_42_23 s_42_22
        let s_42_24: i128 = (s_42_23 + s_42_22);
        // D s_42_25: bit-extract s_42_20 s_42_18 s_42_24
        let s_42_25: Bits = (Bits::new(
            ((s_42_20) >> (s_42_18)).value(),
            u16::try_from(s_42_24).unwrap(),
        ));
        // D s_42_26: cast reint s_42_25 -> u8
        let s_42_26: bool = ((s_42_25.value()) != 0);
        // C s_42_27: const #16984u : u32
        let s_42_27: u32 = 16984;
        // N s_42_28: write-reg s_42_27 <= s_42_26
        let s_42_28: () = {
            state.write_register::<bool>(s_42_27 as isize, s_42_26);
            tracer.write_register(s_42_27 as isize, s_42_26);
        };
        // C s_42_29: const #2s : i
        let s_42_29: i128 = 2;
        // D s_42_30: read-var nzcv:u8
        let s_42_30: u8 = fn_state.nzcv;
        // D s_42_31: cast zx s_42_30 -> bv
        let s_42_31: Bits = Bits::new(s_42_30 as u128, 4u16);
        // C s_42_32: const #1s : i64
        let s_42_32: i64 = 1;
        // C s_42_33: cast zx s_42_32 -> i
        let s_42_33: i128 = (i128::try_from(s_42_32).unwrap());
        // C s_42_34: const #0s : i
        let s_42_34: i128 = 0;
        // C s_42_35: add s_42_34 s_42_33
        let s_42_35: i128 = (s_42_34 + s_42_33);
        // D s_42_36: bit-extract s_42_31 s_42_29 s_42_35
        let s_42_36: Bits = (Bits::new(
            ((s_42_31) >> (s_42_29)).value(),
            u16::try_from(s_42_35).unwrap(),
        ));
        // D s_42_37: cast reint s_42_36 -> u8
        let s_42_37: bool = ((s_42_36.value()) != 0);
        // C s_42_38: const #16997u : u32
        let s_42_38: u32 = 16997;
        // N s_42_39: write-reg s_42_38 <= s_42_37
        let s_42_39: () = {
            state.write_register::<bool>(s_42_38 as isize, s_42_37);
            tracer.write_register(s_42_38 as isize, s_42_37);
        };
        // C s_42_40: const #1s : i
        let s_42_40: i128 = 1;
        // D s_42_41: read-var nzcv:u8
        let s_42_41: u8 = fn_state.nzcv;
        // D s_42_42: cast zx s_42_41 -> bv
        let s_42_42: Bits = Bits::new(s_42_41 as u128, 4u16);
        // C s_42_43: const #1s : i64
        let s_42_43: i64 = 1;
        // C s_42_44: cast zx s_42_43 -> i
        let s_42_44: i128 = (i128::try_from(s_42_43).unwrap());
        // C s_42_45: const #0s : i
        let s_42_45: i128 = 0;
        // C s_42_46: add s_42_45 s_42_44
        let s_42_46: i128 = (s_42_45 + s_42_44);
        // D s_42_47: bit-extract s_42_42 s_42_40 s_42_46
        let s_42_47: Bits = (Bits::new(
            ((s_42_42) >> (s_42_40)).value(),
            u16::try_from(s_42_46).unwrap(),
        ));
        // D s_42_48: cast reint s_42_47 -> u8
        let s_42_48: bool = ((s_42_47.value()) != 0);
        // C s_42_49: const #16971u : u32
        let s_42_49: u32 = 16971;
        // N s_42_50: write-reg s_42_49 <= s_42_48
        let s_42_50: () = {
            state.write_register::<bool>(s_42_49 as isize, s_42_48);
            tracer.write_register(s_42_49 as isize, s_42_48);
        };
        // C s_42_51: const #0s : i
        let s_42_51: i128 = 0;
        // D s_42_52: read-var nzcv:u8
        let s_42_52: u8 = fn_state.nzcv;
        // D s_42_53: cast zx s_42_52 -> bv
        let s_42_53: Bits = Bits::new(s_42_52 as u128, 4u16);
        // C s_42_54: const #1s : i64
        let s_42_54: i64 = 1;
        // C s_42_55: cast zx s_42_54 -> i
        let s_42_55: i128 = (i128::try_from(s_42_54).unwrap());
        // C s_42_56: const #0s : i
        let s_42_56: i128 = 0;
        // C s_42_57: add s_42_56 s_42_55
        let s_42_57: i128 = (s_42_56 + s_42_55);
        // D s_42_58: bit-extract s_42_53 s_42_51 s_42_57
        let s_42_58: Bits = (Bits::new(
            ((s_42_53) >> (s_42_51)).value(),
            u16::try_from(s_42_57).unwrap(),
        ));
        // D s_42_59: cast reint s_42_58 -> u8
        let s_42_59: bool = ((s_42_58.value()) != 0);
        // C s_42_60: const #16996u : u32
        let s_42_60: u32 = 16996;
        // N s_42_61: write-reg s_42_60 <= s_42_59
        let s_42_61: () = {
            state.write_register::<bool>(s_42_60 as isize, s_42_59);
            tracer.write_register(s_42_60 as isize, s_42_59);
        };
        // N s_42_62: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_43_0: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var stagecpysize:u64
        let s_44_0: u64 = fn_state.stagecpysize;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 64u16);
        // D s_44_2: cast sx s_44_1 -> i
        let s_44_2: i128 = {
            let sign_bit = s_44_1.length() - 1;
            let mut result = s_44_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_44_3: cast reint s_44_2 -> i64
        let s_44_3: i64 = (s_44_2 as i64);
        // C s_44_4: const #0s : i
        let s_44_4: i128 = 0;
        // D s_44_5: cast zx s_44_3 -> i
        let s_44_5: i128 = (i128::try_from(s_44_3).unwrap());
        // D s_44_6: call neq_int(s_44_5, s_44_4)
        let s_44_6: bool = neq_int(state, tracer, s_44_5, s_44_4);
        // D s_44_7: not s_44_6
        let s_44_7: bool = !s_44_6;
        // N s_44_8: branch s_44_7 b61 b45
        if s_44_7 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var toaddress:u64
        let s_45_0: u64 = fn_state.toaddress;
        // D s_45_1: read-var fromaddress:u64
        let s_45_1: u64 = fn_state.fromaddress;
        // D s_45_2: read-var cpysize:u64
        let s_45_2: u64 = fn_state.cpysize;
        // D s_45_3: call CPYSizeChoice(s_45_0, s_45_1, s_45_2)
        let s_45_3: i128 = CPYSizeChoice(state, tracer, s_45_0, s_45_1, s_45_2);
        // D s_45_4: write-var B <= s_45_3
        fn_state.B = s_45_3;
        // D s_45_5: read-var B:i
        let s_45_5: i128 = fn_state.B;
        // D s_45_6: write-var Bshadow#1139 <= s_45_5
        fn_state.Bshadow_1139 = s_45_5;
        // D s_45_7: read-var cpysize:u64
        let s_45_7: u64 = fn_state.cpysize;
        // D s_45_8: cast zx s_45_7 -> bv
        let s_45_8: Bits = Bits::new(s_45_7 as u128, 64u16);
        // D s_45_9: cast sx s_45_8 -> i
        let s_45_9: i128 = {
            let sign_bit = s_45_8.length() - 1;
            let mut result = s_45_8.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_45_10: cast reint s_45_9 -> i64
        let s_45_10: i64 = (s_45_9 as i64);
        // C s_45_11: const #0s : i
        let s_45_11: i128 = 0;
        // D s_45_12: cast zx s_45_10 -> i
        let s_45_12: i128 = (i128::try_from(s_45_10).unwrap());
        // D s_45_13: cmp-lt s_45_12 s_45_11
        let s_45_13: bool = ((s_45_12) < (s_45_11));
        // N s_45_14: branch s_45_13 b55 b46
        if s_45_13 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var stagecpysize:u64
        let s_46_0: u64 = fn_state.stagecpysize;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 64u16);
        // D s_46_2: cast sx s_46_1 -> i
        let s_46_2: i128 = {
            let sign_bit = s_46_1.length() - 1;
            let mut result = s_46_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_46_3: cast reint s_46_2 -> i64
        let s_46_3: i64 = (s_46_2 as i64);
        // D s_46_4: cast zx s_46_3 -> i
        let s_46_4: i128 = (i128::try_from(s_46_3).unwrap());
        // D s_46_5: read-var Bshadow#1139:i
        let s_46_5: i128 = fn_state.Bshadow_1139;
        // D s_46_6: cmp-le s_46_5 s_46_4
        let s_46_6: bool = ((s_46_5) <= (s_46_4));
        // N s_46_7: assert s_46_6
        let s_46_7: () = assert!(s_46_6);
        // D s_46_8: read-var cpysize:u64
        let s_46_8: u64 = fn_state.cpysize;
        // D s_46_9: cast zx s_46_8 -> bv
        let s_46_9: Bits = Bits::new(s_46_8 as u128, 64u16);
        // D s_46_10: read-var Bshadow#1139:i
        let s_46_10: i128 = fn_state.Bshadow_1139;
        // D s_46_11: cast cvt s_46_10 -> bv
        let s_46_11: Bits = Bits::new(s_46_10 as u128, 128);
        // D s_46_12: sub s_46_9 s_46_11
        let s_46_12: Bits = ((s_46_9) - (s_46_11));
        // D s_46_13: cast reint s_46_12 -> u64
        let s_46_13: u64 = (s_46_12.value() as u64);
        // D s_46_14: write-var cpysize <= s_46_13
        fn_state.cpysize = s_46_13;
        // D s_46_15: read-var stagecpysize:u64
        let s_46_15: u64 = fn_state.stagecpysize;
        // D s_46_16: cast zx s_46_15 -> bv
        let s_46_16: Bits = Bits::new(s_46_15 as u128, 64u16);
        // D s_46_17: read-var Bshadow#1139:i
        let s_46_17: i128 = fn_state.Bshadow_1139;
        // D s_46_18: cast cvt s_46_17 -> bv
        let s_46_18: Bits = Bits::new(s_46_17 as u128, 128);
        // D s_46_19: sub s_46_16 s_46_18
        let s_46_19: Bits = ((s_46_16) - (s_46_18));
        // D s_46_20: cast reint s_46_19 -> u64
        let s_46_20: u64 = (s_46_19.value() as u64);
        // D s_46_21: write-var stagecpysize <= s_46_20
        fn_state.stagecpysize = s_46_20;
        // D s_46_22: read-var Bshadow#1139:i
        let s_46_22: i128 = fn_state.Bshadow_1139;
        // D s_46_23: call __id(s_46_22)
        let s_46_23: i128 = u__id(state, tracer, s_46_22);
        // C s_46_24: const #8s : i
        let s_46_24: i128 = 8;
        // D s_46_25: mul s_46_23 s_46_24
        let s_46_25: i128 = ((s_46_23) * (s_46_24));
        // C s_46_26: const #1s : i
        let s_46_26: i128 = 1;
        // D s_46_27: sub s_46_25 s_46_26
        let s_46_27: i128 = ((s_46_25) - (s_46_26));
        // C s_46_28: const #0s : i
        let s_46_28: i128 = 0;
        // D s_46_29: cmp-le s_46_28 s_46_27
        let s_46_29: bool = ((s_46_28) <= (s_46_27));
        // N s_46_30: branch s_46_29 b54 b47
        if s_46_29 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#146530 <= s_47_0
        fn_state.gs_146530 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#146530:u8
        let s_48_0: bool = fn_state.gs_146530;
        // N s_48_1: assert s_48_0
        let s_48_1: () = assert!(s_48_0);
        // C s_48_2: const #8s : i
        let s_48_2: i128 = 8;
        // D s_48_3: read-var Bshadow#1139:i
        let s_48_3: i128 = fn_state.Bshadow_1139;
        // D s_48_4: mul s_48_3 s_48_2
        let s_48_4: i128 = ((s_48_3) * (s_48_2));
        // C s_48_5: const #1s : i
        let s_48_5: i128 = 1;
        // D s_48_6: sub s_48_4 s_48_5
        let s_48_6: i128 = ((s_48_4) - (s_48_5));
        // D s_48_7: write-var ga#251505 <= s_48_6
        fn_state.ga_251505 = s_48_6;
        // D s_48_8: read-var fromaddress:u64
        let s_48_8: u64 = fn_state.fromaddress;
        // D s_48_9: cast zx s_48_8 -> bv
        let s_48_9: Bits = Bits::new(s_48_8 as u128, 64u16);
        // D s_48_10: read-var cpysize:u64
        let s_48_10: u64 = fn_state.cpysize;
        // D s_48_11: cast zx s_48_10 -> bv
        let s_48_11: Bits = Bits::new(s_48_10 as u128, 64u16);
        // D s_48_12: add s_48_9 s_48_11
        let s_48_12: Bits = (s_48_9 + s_48_11);
        // D s_48_13: cast reint s_48_12 -> u64
        let s_48_13: u64 = (s_48_12.value() as u64);
        // D s_48_14: read-var Bshadow#1139:i
        let s_48_14: i128 = fn_state.Bshadow_1139;
        // D s_48_15: read-var raccdesc:struct
        let s_48_15: ProductType9878976b5bcce9c9 = fn_state.raccdesc;
        // D s_48_16: call Mem_read(s_48_13, s_48_14, s_48_15)
        let s_48_16: Bits = Mem_read(state, tracer, s_48_13, s_48_14, s_48_15);
        // D s_48_17: write-var ga#251506 <= s_48_16
        fn_state.ga_251506 = s_48_16;
        // N s_48_18: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0s : i
        let s_49_0: i128 = 0;
        // D s_49_1: read-var readdata:bv
        let s_49_1: Bits = fn_state.readdata;
        // D s_49_2: read-var ga#251505:i
        let s_49_2: i128 = fn_state.ga_251505;
        // D s_49_3: read-var ga#251506:bv
        let s_49_3: Bits = fn_state.ga_251506;
        // D s_49_4: sub s_49_2 s_49_0
        let s_49_4: i128 = ((s_49_2) - (s_49_0));
        // C s_49_5: const #1u : u64
        let s_49_5: u64 = 1;
        // C s_49_6: cast zx s_49_5 -> bv
        let s_49_6: Bits = Bits::new(s_49_5 as u128, 64u16);
        // D s_49_7: lsl s_49_6 s_49_4
        let s_49_7: Bits = s_49_6 << s_49_4;
        // D s_49_8: sub s_49_7 s_49_6
        let s_49_8: Bits = ((s_49_7) - (s_49_6));
        // D s_49_9: and s_49_3 s_49_8
        let s_49_9: Bits = ((s_49_3) & (s_49_8));
        // D s_49_10: lsl s_49_9 s_49_0
        let s_49_10: Bits = s_49_9 << s_49_0;
        // D s_49_11: lsl s_49_8 s_49_0
        let s_49_11: Bits = s_49_8 << s_49_0;
        // D s_49_12: cmpl s_49_11
        let s_49_12: Bits = !s_49_11;
        // D s_49_13: and s_49_1 s_49_12
        let s_49_13: Bits = ((s_49_1) & (s_49_12));
        // D s_49_14: or s_49_13 s_49_10
        let s_49_14: Bits = ((s_49_13) | (s_49_10));
        // D s_49_15: write-var readdata <= s_49_14
        fn_state.readdata = s_49_14;
        // D s_49_16: read-var toaddress:u64
        let s_49_16: u64 = fn_state.toaddress;
        // D s_49_17: cast zx s_49_16 -> bv
        let s_49_17: Bits = Bits::new(s_49_16 as u128, 64u16);
        // D s_49_18: read-var cpysize:u64
        let s_49_18: u64 = fn_state.cpysize;
        // D s_49_19: cast zx s_49_18 -> bv
        let s_49_19: Bits = Bits::new(s_49_18 as u128, 64u16);
        // D s_49_20: add s_49_17 s_49_19
        let s_49_20: Bits = (s_49_17 + s_49_19);
        // D s_49_21: cast reint s_49_20 -> u64
        let s_49_21: u64 = (s_49_20.value() as u64);
        // C s_49_22: const #8s : i
        let s_49_22: i128 = 8;
        // D s_49_23: read-var Bshadow#1139:i
        let s_49_23: i128 = fn_state.Bshadow_1139;
        // D s_49_24: mul s_49_23 s_49_22
        let s_49_24: i128 = ((s_49_23) * (s_49_22));
        // C s_49_25: const #1s : i
        let s_49_25: i128 = 1;
        // D s_49_26: sub s_49_24 s_49_25
        let s_49_26: i128 = ((s_49_24) - (s_49_25));
        // C s_49_27: const #0s : i
        let s_49_27: i128 = 0;
        // D s_49_28: read-var readdata:bv
        let s_49_28: Bits = fn_state.readdata;
        // C s_49_29: const #1s : i64
        let s_49_29: i64 = 1;
        // C s_49_30: cast zx s_49_29 -> i
        let s_49_30: i128 = (i128::try_from(s_49_29).unwrap());
        // D s_49_31: sub s_49_26 s_49_27
        let s_49_31: i128 = ((s_49_26) - (s_49_27));
        // D s_49_32: add s_49_31 s_49_30
        let s_49_32: i128 = (s_49_31 + s_49_30);
        // D s_49_33: bit-extract s_49_28 s_49_27 s_49_32
        let s_49_33: Bits = (Bits::new(
            ((s_49_28) >> (s_49_27)).value(),
            u16::try_from(s_49_32).unwrap(),
        ));
        // D s_49_34: read-var Bshadow#1139:i
        let s_49_34: i128 = fn_state.Bshadow_1139;
        // D s_49_35: read-var waccdesc:struct
        let s_49_35: ProductType9878976b5bcce9c9 = fn_state.waccdesc;
        // D s_49_36: call Mem_set(s_49_21, s_49_34, s_49_35, s_49_33)
        let s_49_36: () = Mem_set(state, tracer, s_49_21, s_49_34, s_49_35, s_49_33);
        // N s_49_37: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var stage:u32
        let s_50_0: u32 = fn_state.stage;
        // C s_50_1: const #0u : u32
        let s_50_1: u32 = 0;
        // D s_50_2: cmp-eq s_50_0 s_50_1
        let s_50_2: bool = ((s_50_0) == (s_50_1));
        // N s_50_3: branch s_50_2 b53 b51
        if s_50_2 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_51_0: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_52_0: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #64s : i64
        let s_53_0: i64 = 64;
        // D s_53_1: read-var n:i64
        let s_53_1: i64 = fn_state.n;
        // D s_53_2: cast zx s_53_1 -> i
        let s_53_2: i128 = (i128::try_from(s_53_1).unwrap());
        // D s_53_3: read-var cpysize:u64
        let s_53_3: u64 = fn_state.cpysize;
        // D s_53_4: cast zx s_53_3 -> bv
        let s_53_4: Bits = Bits::new(s_53_3 as u128, 64u16);
        // D s_53_5: call X_set(s_53_2, s_53_0, s_53_4)
        let s_53_5: () = X_set(state, tracer, s_53_2, s_53_0, s_53_4);
        // N s_53_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var Bshadow#1139:i
        let s_54_0: i128 = fn_state.Bshadow_1139;
        // D s_54_1: call __id(s_54_0)
        let s_54_1: i128 = u__id(state, tracer, s_54_0);
        // C s_54_2: const #8s : i
        let s_54_2: i128 = 8;
        // D s_54_3: mul s_54_1 s_54_2
        let s_54_3: i128 = ((s_54_1) * (s_54_2));
        // C s_54_4: const #1s : i
        let s_54_4: i128 = 1;
        // D s_54_5: sub s_54_3 s_54_4
        let s_54_5: i128 = ((s_54_3) - (s_54_4));
        // D s_54_6: read-var N:i
        let s_54_6: i128 = fn_state.N;
        // D s_54_7: call __id(s_54_6)
        let s_54_7: i128 = u__id(state, tracer, s_54_6);
        // C s_54_8: const #8s : i
        let s_54_8: i128 = 8;
        // D s_54_9: mul s_54_8 s_54_7
        let s_54_9: i128 = ((s_54_8) * (s_54_7));
        // D s_54_10: cmp-lt s_54_5 s_54_9
        let s_54_10: bool = ((s_54_5) < (s_54_9));
        // D s_54_11: write-var gs#146530 <= s_54_10
        fn_state.gs_146530 = s_54_10;
        // N s_54_12: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #1s : i
        let s_55_0: i128 = 1;
        // C s_55_1: neg s_55_0
        let s_55_1: i128 = -s_55_0;
        // C s_55_2: cast reint s_55_1 -> i64
        let s_55_2: i64 = (s_55_1 as i64);
        // D s_55_3: read-var stagecpysize:u64
        let s_55_3: u64 = fn_state.stagecpysize;
        // D s_55_4: cast zx s_55_3 -> bv
        let s_55_4: Bits = Bits::new(s_55_3 as u128, 64u16);
        // D s_55_5: cast sx s_55_4 -> i
        let s_55_5: i128 = {
            let sign_bit = s_55_4.length() - 1;
            let mut result = s_55_4.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_55_6: cast reint s_55_5 -> i64
        let s_55_6: i64 = (s_55_5 as i64);
        // C s_55_7: cast zx s_55_2 -> i
        let s_55_7: i128 = (i128::try_from(s_55_2).unwrap());
        // D s_55_8: cast zx s_55_6 -> i
        let s_55_8: i128 = (i128::try_from(s_55_6).unwrap());
        // D s_55_9: mul s_55_7 s_55_8
        let s_55_9: i128 = ((s_55_7) * (s_55_8));
        // D s_55_10: read-var Bshadow#1139:i
        let s_55_10: i128 = fn_state.Bshadow_1139;
        // D s_55_11: cmp-le s_55_10 s_55_9
        let s_55_11: bool = ((s_55_10) <= (s_55_9));
        // N s_55_12: assert s_55_11
        let s_55_12: () = assert!(s_55_11);
        // D s_55_13: read-var Bshadow#1139:i
        let s_55_13: i128 = fn_state.Bshadow_1139;
        // D s_55_14: call __id(s_55_13)
        let s_55_14: i128 = u__id(state, tracer, s_55_13);
        // C s_55_15: const #8s : i
        let s_55_15: i128 = 8;
        // D s_55_16: mul s_55_14 s_55_15
        let s_55_16: i128 = ((s_55_14) * (s_55_15));
        // C s_55_17: const #1s : i
        let s_55_17: i128 = 1;
        // D s_55_18: sub s_55_16 s_55_17
        let s_55_18: i128 = ((s_55_16) - (s_55_17));
        // C s_55_19: const #0s : i
        let s_55_19: i128 = 0;
        // D s_55_20: cmp-le s_55_19 s_55_18
        let s_55_20: bool = ((s_55_19) <= (s_55_18));
        // N s_55_21: branch s_55_20 b60 b56
        if s_55_20 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var gs#146549 <= s_56_0
        fn_state.gs_146549 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#146549:u8
        let s_57_0: bool = fn_state.gs_146549;
        // N s_57_1: assert s_57_0
        let s_57_1: () = assert!(s_57_0);
        // C s_57_2: const #8s : i
        let s_57_2: i128 = 8;
        // D s_57_3: read-var Bshadow#1139:i
        let s_57_3: i128 = fn_state.Bshadow_1139;
        // D s_57_4: mul s_57_3 s_57_2
        let s_57_4: i128 = ((s_57_3) * (s_57_2));
        // C s_57_5: const #1s : i
        let s_57_5: i128 = 1;
        // D s_57_6: sub s_57_4 s_57_5
        let s_57_6: i128 = ((s_57_4) - (s_57_5));
        // D s_57_7: write-var ga#251485 <= s_57_6
        fn_state.ga_251485 = s_57_6;
        // D s_57_8: read-var fromaddress:u64
        let s_57_8: u64 = fn_state.fromaddress;
        // D s_57_9: cast zx s_57_8 -> bv
        let s_57_9: Bits = Bits::new(s_57_8 as u128, 64u16);
        // D s_57_10: read-var cpysize:u64
        let s_57_10: u64 = fn_state.cpysize;
        // D s_57_11: cast zx s_57_10 -> bv
        let s_57_11: Bits = Bits::new(s_57_10 as u128, 64u16);
        // D s_57_12: add s_57_9 s_57_11
        let s_57_12: Bits = (s_57_9 + s_57_11);
        // D s_57_13: cast reint s_57_12 -> u64
        let s_57_13: u64 = (s_57_12.value() as u64);
        // D s_57_14: read-var Bshadow#1139:i
        let s_57_14: i128 = fn_state.Bshadow_1139;
        // D s_57_15: read-var raccdesc:struct
        let s_57_15: ProductType9878976b5bcce9c9 = fn_state.raccdesc;
        // D s_57_16: call Mem_read(s_57_13, s_57_14, s_57_15)
        let s_57_16: Bits = Mem_read(state, tracer, s_57_13, s_57_14, s_57_15);
        // D s_57_17: write-var ga#251486 <= s_57_16
        fn_state.ga_251486 = s_57_16;
        // N s_57_18: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #0s : i
        let s_58_0: i128 = 0;
        // D s_58_1: read-var readdata:bv
        let s_58_1: Bits = fn_state.readdata;
        // D s_58_2: read-var ga#251485:i
        let s_58_2: i128 = fn_state.ga_251485;
        // D s_58_3: read-var ga#251486:bv
        let s_58_3: Bits = fn_state.ga_251486;
        // D s_58_4: sub s_58_2 s_58_0
        let s_58_4: i128 = ((s_58_2) - (s_58_0));
        // C s_58_5: const #1u : u64
        let s_58_5: u64 = 1;
        // C s_58_6: cast zx s_58_5 -> bv
        let s_58_6: Bits = Bits::new(s_58_5 as u128, 64u16);
        // D s_58_7: lsl s_58_6 s_58_4
        let s_58_7: Bits = s_58_6 << s_58_4;
        // D s_58_8: sub s_58_7 s_58_6
        let s_58_8: Bits = ((s_58_7) - (s_58_6));
        // D s_58_9: and s_58_3 s_58_8
        let s_58_9: Bits = ((s_58_3) & (s_58_8));
        // D s_58_10: lsl s_58_9 s_58_0
        let s_58_10: Bits = s_58_9 << s_58_0;
        // D s_58_11: lsl s_58_8 s_58_0
        let s_58_11: Bits = s_58_8 << s_58_0;
        // D s_58_12: cmpl s_58_11
        let s_58_12: Bits = !s_58_11;
        // D s_58_13: and s_58_1 s_58_12
        let s_58_13: Bits = ((s_58_1) & (s_58_12));
        // D s_58_14: or s_58_13 s_58_10
        let s_58_14: Bits = ((s_58_13) | (s_58_10));
        // D s_58_15: write-var readdata <= s_58_14
        fn_state.readdata = s_58_14;
        // D s_58_16: read-var toaddress:u64
        let s_58_16: u64 = fn_state.toaddress;
        // D s_58_17: cast zx s_58_16 -> bv
        let s_58_17: Bits = Bits::new(s_58_16 as u128, 64u16);
        // D s_58_18: read-var cpysize:u64
        let s_58_18: u64 = fn_state.cpysize;
        // D s_58_19: cast zx s_58_18 -> bv
        let s_58_19: Bits = Bits::new(s_58_18 as u128, 64u16);
        // D s_58_20: add s_58_17 s_58_19
        let s_58_20: Bits = (s_58_17 + s_58_19);
        // D s_58_21: cast reint s_58_20 -> u64
        let s_58_21: u64 = (s_58_20.value() as u64);
        // C s_58_22: const #8s : i
        let s_58_22: i128 = 8;
        // D s_58_23: read-var Bshadow#1139:i
        let s_58_23: i128 = fn_state.Bshadow_1139;
        // D s_58_24: mul s_58_23 s_58_22
        let s_58_24: i128 = ((s_58_23) * (s_58_22));
        // C s_58_25: const #1s : i
        let s_58_25: i128 = 1;
        // D s_58_26: sub s_58_24 s_58_25
        let s_58_26: i128 = ((s_58_24) - (s_58_25));
        // C s_58_27: const #0s : i
        let s_58_27: i128 = 0;
        // D s_58_28: read-var readdata:bv
        let s_58_28: Bits = fn_state.readdata;
        // C s_58_29: const #1s : i64
        let s_58_29: i64 = 1;
        // C s_58_30: cast zx s_58_29 -> i
        let s_58_30: i128 = (i128::try_from(s_58_29).unwrap());
        // D s_58_31: sub s_58_26 s_58_27
        let s_58_31: i128 = ((s_58_26) - (s_58_27));
        // D s_58_32: add s_58_31 s_58_30
        let s_58_32: i128 = (s_58_31 + s_58_30);
        // D s_58_33: bit-extract s_58_28 s_58_27 s_58_32
        let s_58_33: Bits = (Bits::new(
            ((s_58_28) >> (s_58_27)).value(),
            u16::try_from(s_58_32).unwrap(),
        ));
        // D s_58_34: read-var Bshadow#1139:i
        let s_58_34: i128 = fn_state.Bshadow_1139;
        // D s_58_35: read-var waccdesc:struct
        let s_58_35: ProductType9878976b5bcce9c9 = fn_state.waccdesc;
        // D s_58_36: call Mem_set(s_58_21, s_58_34, s_58_35, s_58_33)
        let s_58_36: () = Mem_set(state, tracer, s_58_21, s_58_34, s_58_35, s_58_33);
        // N s_58_37: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var cpysize:u64
        let s_59_0: u64 = fn_state.cpysize;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 64u16);
        // D s_59_2: read-var Bshadow#1139:i
        let s_59_2: i128 = fn_state.Bshadow_1139;
        // D s_59_3: cast cvt s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 128);
        // D s_59_4: add s_59_1 s_59_3
        let s_59_4: Bits = (s_59_1 + s_59_3);
        // D s_59_5: cast reint s_59_4 -> u64
        let s_59_5: u64 = (s_59_4.value() as u64);
        // D s_59_6: write-var cpysize <= s_59_5
        fn_state.cpysize = s_59_5;
        // D s_59_7: read-var stagecpysize:u64
        let s_59_7: u64 = fn_state.stagecpysize;
        // D s_59_8: cast zx s_59_7 -> bv
        let s_59_8: Bits = Bits::new(s_59_7 as u128, 64u16);
        // D s_59_9: read-var Bshadow#1139:i
        let s_59_9: i128 = fn_state.Bshadow_1139;
        // D s_59_10: cast cvt s_59_9 -> bv
        let s_59_10: Bits = Bits::new(s_59_9 as u128, 128);
        // D s_59_11: add s_59_8 s_59_10
        let s_59_11: Bits = (s_59_8 + s_59_10);
        // D s_59_12: cast reint s_59_11 -> u64
        let s_59_12: u64 = (s_59_11.value() as u64);
        // D s_59_13: write-var stagecpysize <= s_59_12
        fn_state.stagecpysize = s_59_12;
        // N s_59_14: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var Bshadow#1139:i
        let s_60_0: i128 = fn_state.Bshadow_1139;
        // D s_60_1: call __id(s_60_0)
        let s_60_1: i128 = u__id(state, tracer, s_60_0);
        // C s_60_2: const #8s : i
        let s_60_2: i128 = 8;
        // D s_60_3: mul s_60_1 s_60_2
        let s_60_3: i128 = ((s_60_1) * (s_60_2));
        // C s_60_4: const #1s : i
        let s_60_4: i128 = 1;
        // D s_60_5: sub s_60_3 s_60_4
        let s_60_5: i128 = ((s_60_3) - (s_60_4));
        // D s_60_6: read-var N:i
        let s_60_6: i128 = fn_state.N;
        // D s_60_7: call __id(s_60_6)
        let s_60_7: i128 = u__id(state, tracer, s_60_6);
        // C s_60_8: const #8s : i
        let s_60_8: i128 = 8;
        // D s_60_9: mul s_60_8 s_60_7
        let s_60_9: i128 = ((s_60_8) * (s_60_7));
        // D s_60_10: cmp-lt s_60_5 s_60_9
        let s_60_10: bool = ((s_60_5) < (s_60_9));
        // D s_60_11: write-var gs#146549 <= s_60_10
        fn_state.gs_146549 = s_60_10;
        // N s_60_12: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_61_0: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #() : ()
        let s_62_0: () = ();
        // S s_62_1: call SPESampleMemCopy(s_62_0)
        let s_62_1: () = SPESampleMemCopy(state, tracer, s_62_0);
        // N s_62_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // C s_63_1: const #1u : u8
        let s_63_1: bool = true;
        // D s_63_2: read-var d:i64
        let s_63_2: i64 = fn_state.d;
        // D s_63_3: cast zx s_63_2 -> i
        let s_63_3: i128 = (i128::try_from(s_63_2).unwrap());
        // D s_63_4: read-var s:i64
        let s_63_4: i64 = fn_state.s;
        // D s_63_5: cast zx s_63_4 -> i
        let s_63_5: i128 = (i128::try_from(s_63_4).unwrap());
        // D s_63_6: read-var n:i64
        let s_63_6: i64 = fn_state.n;
        // D s_63_7: cast zx s_63_6 -> i
        let s_63_7: i128 = (i128::try_from(s_63_6).unwrap());
        // D s_63_8: read-var supports_option_a:u8
        let s_63_8: bool = fn_state.supports_option_a;
        // D s_63_9: read-var options_name:u8
        let s_63_9: u8 = fn_state.options_name;
        // D s_63_10: call MismatchedMemCpyException(s_63_8, s_63_3, s_63_5, s_63_7, s_63_0, s_63_1, s_63_9)
        let s_63_10: () = MismatchedMemCpyException(
            state,
            tracer,
            s_63_8,
            s_63_3,
            s_63_5,
            s_63_7,
            s_63_0,
            s_63_1,
            s_63_9,
        );
        // N s_63_11: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #1u : u8
        let s_64_0: bool = true;
        // D s_64_1: write-var gs#146585 <= s_64_0
        fn_state.gs_146585 = s_64_0;
        // N s_64_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var cpysize:u64
        let s_65_0: u64 = fn_state.cpysize;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 64u16);
        // D s_65_2: read-var postsize:u64
        let s_65_2: u64 = fn_state.postsize;
        // D s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 64u16);
        // D s_65_4: sub s_65_1 s_65_3
        let s_65_4: Bits = ((s_65_1) - (s_65_3));
        // D s_65_5: cast reint s_65_4 -> u64
        let s_65_5: u64 = (s_65_4.value() as u64);
        // D s_65_6: write-var stagecpysize <= s_65_5
        fn_state.stagecpysize = s_65_5;
        // D s_65_7: read-var toaddress:u64
        let s_65_7: u64 = fn_state.toaddress;
        // D s_65_8: read-var fromaddress:u64
        let s_65_8: u64 = fn_state.fromaddress;
        // D s_65_9: read-var cpysize:u64
        let s_65_9: u64 = fn_state.cpysize;
        // D s_65_10: call MemCpyParametersIllformedM(s_65_7, s_65_8, s_65_9)
        let s_65_10: bool = MemCpyParametersIllformedM(
            state,
            tracer,
            s_65_7,
            s_65_8,
            s_65_9,
        );
        // N s_65_11: branch s_65_10 b68 b66
        if s_65_10 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_66_0: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_67_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #0u : u8
        let s_68_0: bool = false;
        // C s_68_1: const #0u : u8
        let s_68_1: bool = false;
        // D s_68_2: read-var d:i64
        let s_68_2: i64 = fn_state.d;
        // D s_68_3: cast zx s_68_2 -> i
        let s_68_3: i128 = (i128::try_from(s_68_2).unwrap());
        // D s_68_4: read-var s:i64
        let s_68_4: i64 = fn_state.s;
        // D s_68_5: cast zx s_68_4 -> i
        let s_68_5: i128 = (i128::try_from(s_68_4).unwrap());
        // D s_68_6: read-var n:i64
        let s_68_6: i64 = fn_state.n;
        // D s_68_7: cast zx s_68_6 -> i
        let s_68_7: i128 = (i128::try_from(s_68_6).unwrap());
        // D s_68_8: read-var supports_option_a:u8
        let s_68_8: bool = fn_state.supports_option_a;
        // D s_68_9: read-var options_name:u8
        let s_68_9: u8 = fn_state.options_name;
        // D s_68_10: call MismatchedMemCpyException(s_68_8, s_68_3, s_68_5, s_68_7, s_68_0, s_68_1, s_68_9)
        let s_68_10: () = MismatchedMemCpyException(
            state,
            tracer,
            s_68_8,
            s_68_3,
            s_68_5,
            s_68_7,
            s_68_0,
            s_68_1,
            s_68_9,
        );
        // N s_68_11: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #1u : u8
        let s_69_0: bool = true;
        // D s_69_1: write-var gs#146583 <= s_69_0
        fn_state.gs_146583 = s_69_0;
        // N s_69_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var supports_option_a:u8
        let s_70_0: bool = fn_state.supports_option_a;
        // N s_70_1: branch s_70_0 b75 b71
        if s_70_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #1s : i
        let s_71_0: i128 = 1;
        // D s_71_1: read-var nzcv:u8
        let s_71_1: u8 = fn_state.nzcv;
        // D s_71_2: cast zx s_71_1 -> bv
        let s_71_2: Bits = Bits::new(s_71_1 as u128, 4u16);
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
        // C s_71_19: const #0u : u8
        let s_71_19: bool = false;
        // C s_71_20: cast zx s_71_19 -> bv
        let s_71_20: Bits = Bits::new(s_71_19 as u128, 1u16);
        // D s_71_21: cmp-eq s_71_18 s_71_20
        let s_71_21: bool = ((s_71_18) == (s_71_20));
        // N s_71_22: branch s_71_21 b74 b72
        if s_71_21 {
            return block_74(state, tracer, fn_state);
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
        // N s_73_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #1u : u8
        let s_74_0: bool = true;
        // D s_74_1: read-var stage:u32
        let s_74_1: u32 = fn_state.stage;
        // C s_74_2: const #2u : u32
        let s_74_2: u32 = 2;
        // D s_74_3: cmp-eq s_74_1 s_74_2
        let s_74_3: bool = ((s_74_1) == (s_74_2));
        // D s_74_4: read-var d:i64
        let s_74_4: i64 = fn_state.d;
        // D s_74_5: cast zx s_74_4 -> i
        let s_74_5: i128 = (i128::try_from(s_74_4).unwrap());
        // D s_74_6: read-var s:i64
        let s_74_6: i64 = fn_state.s;
        // D s_74_7: cast zx s_74_6 -> i
        let s_74_7: i128 = (i128::try_from(s_74_6).unwrap());
        // D s_74_8: read-var n:i64
        let s_74_8: i64 = fn_state.n;
        // D s_74_9: cast zx s_74_8 -> i
        let s_74_9: i128 = (i128::try_from(s_74_8).unwrap());
        // D s_74_10: read-var supports_option_a:u8
        let s_74_10: bool = fn_state.supports_option_a;
        // D s_74_11: read-var options_name:u8
        let s_74_11: u8 = fn_state.options_name;
        // D s_74_12: call MismatchedMemCpyException(s_74_10, s_74_5, s_74_7, s_74_9, s_74_0, s_74_3, s_74_11)
        let s_74_12: () = MismatchedMemCpyException(
            state,
            tracer,
            s_74_10,
            s_74_5,
            s_74_7,
            s_74_9,
            s_74_0,
            s_74_3,
            s_74_11,
        );
        // N s_74_13: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #1s : i
        let s_75_0: i128 = 1;
        // D s_75_1: read-var nzcv:u8
        let s_75_1: u8 = fn_state.nzcv;
        // D s_75_2: cast zx s_75_1 -> bv
        let s_75_2: Bits = Bits::new(s_75_1 as u128, 4u16);
        // C s_75_3: const #1u : u64
        let s_75_3: u64 = 1;
        // D s_75_4: bit-extract s_75_2 s_75_0 s_75_3
        let s_75_4: Bits = (Bits::new(
            ((s_75_2) >> (s_75_0)).value(),
            u16::try_from(s_75_3).unwrap(),
        ));
        // D s_75_5: cast reint s_75_4 -> u8
        let s_75_5: bool = ((s_75_4.value()) != 0);
        // C s_75_6: const #0s : i
        let s_75_6: i128 = 0;
        // C s_75_7: const #0u : u64
        let s_75_7: u64 = 0;
        // D s_75_8: cast zx s_75_5 -> u64
        let s_75_8: u64 = (s_75_5 as u64);
        // C s_75_9: const #1u : u64
        let s_75_9: u64 = 1;
        // D s_75_10: and s_75_8 s_75_9
        let s_75_10: u64 = ((s_75_8) & (s_75_9));
        // D s_75_11: cmp-eq s_75_10 s_75_9
        let s_75_11: bool = ((s_75_10) == (s_75_9));
        // D s_75_12: lsl s_75_8 s_75_6
        let s_75_12: u64 = s_75_8 << s_75_6;
        // D s_75_13: or s_75_7 s_75_12
        let s_75_13: u64 = ((s_75_7) | (s_75_12));
        // D s_75_14: cmpl s_75_12
        let s_75_14: u64 = !s_75_12;
        // D s_75_15: and s_75_7 s_75_14
        let s_75_15: u64 = ((s_75_7) & (s_75_14));
        // D s_75_16: select s_75_11 s_75_13 s_75_15
        let s_75_16: u64 = if s_75_11 { s_75_13 } else { s_75_15 };
        // D s_75_17: cast trunc s_75_16 -> u8
        let s_75_17: bool = ((s_75_16) != 0);
        // D s_75_18: cast zx s_75_17 -> bv
        let s_75_18: Bits = Bits::new(s_75_17 as u128, 1u16);
        // C s_75_19: const #1u : u8
        let s_75_19: bool = true;
        // C s_75_20: cast zx s_75_19 -> bv
        let s_75_20: Bits = Bits::new(s_75_19 as u128, 1u16);
        // D s_75_21: cmp-eq s_75_18 s_75_20
        let s_75_21: bool = ((s_75_18) == (s_75_20));
        // N s_75_22: branch s_75_21 b78 b76
        if s_75_21 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_76_0: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_77_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #1u : u8
        let s_78_0: bool = true;
        // D s_78_1: read-var stage:u32
        let s_78_1: u32 = fn_state.stage;
        // C s_78_2: const #2u : u32
        let s_78_2: u32 = 2;
        // D s_78_3: cmp-eq s_78_1 s_78_2
        let s_78_3: bool = ((s_78_1) == (s_78_2));
        // D s_78_4: read-var d:i64
        let s_78_4: i64 = fn_state.d;
        // D s_78_5: cast zx s_78_4 -> i
        let s_78_5: i128 = (i128::try_from(s_78_4).unwrap());
        // D s_78_6: read-var s:i64
        let s_78_6: i64 = fn_state.s;
        // D s_78_7: cast zx s_78_6 -> i
        let s_78_7: i128 = (i128::try_from(s_78_6).unwrap());
        // D s_78_8: read-var n:i64
        let s_78_8: i64 = fn_state.n;
        // D s_78_9: cast zx s_78_8 -> i
        let s_78_9: i128 = (i128::try_from(s_78_8).unwrap());
        // D s_78_10: read-var supports_option_a:u8
        let s_78_10: bool = fn_state.supports_option_a;
        // D s_78_11: read-var options_name:u8
        let s_78_11: u8 = fn_state.options_name;
        // D s_78_12: call MismatchedMemCpyException(s_78_10, s_78_5, s_78_7, s_78_9, s_78_0, s_78_3, s_78_11)
        let s_78_12: () = MismatchedMemCpyException(
            state,
            tracer,
            s_78_10,
            s_78_5,
            s_78_7,
            s_78_9,
            s_78_0,
            s_78_3,
            s_78_11,
        );
        // N s_78_13: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #1u : u8
        let s_79_0: bool = true;
        // D s_79_1: write-var gs#146570 <= s_79_0
        fn_state.gs_146570 = s_79_0;
        // N s_79_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #55s : i
        let s_80_0: i128 = 55;
        // D s_80_1: read-var cpysize:u64
        let s_80_1: u64 = fn_state.cpysize;
        // D s_80_2: cast zx s_80_1 -> bv
        let s_80_2: Bits = Bits::new(s_80_1 as u128, 64u16);
        // C s_80_3: const #1s : i64
        let s_80_3: i64 = 1;
        // C s_80_4: cast zx s_80_3 -> i
        let s_80_4: i128 = (i128::try_from(s_80_3).unwrap());
        // C s_80_5: const #8s : i
        let s_80_5: i128 = 8;
        // C s_80_6: add s_80_5 s_80_4
        let s_80_6: i128 = (s_80_5 + s_80_4);
        // D s_80_7: bit-extract s_80_2 s_80_0 s_80_6
        let s_80_7: Bits = (Bits::new(
            ((s_80_2) >> (s_80_0)).value(),
            u16::try_from(s_80_6).unwrap(),
        ));
        // D s_80_8: cast reint s_80_7 -> u9
        let s_80_8: u16 = (s_80_7.value() as u16);
        // D s_80_9: cast zx s_80_8 -> bv
        let s_80_9: Bits = Bits::new(s_80_8 as u128, 9u16);
        // C s_80_10: const #0u : u9
        let s_80_10: u16 = 0;
        // C s_80_11: cast zx s_80_10 -> bv
        let s_80_11: Bits = Bits::new(s_80_10 as u128, 9u16);
        // D s_80_12: cmp-ne s_80_9 s_80_11
        let s_80_12: bool = ((s_80_9) != (s_80_11));
        // N s_80_13: branch s_80_12 b107 b81
        if s_80_12 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_81_0: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #0s : i
        let s_82_0: i128 = 0;
        // D s_82_1: read-var fromaddress:u64
        let s_82_1: u64 = fn_state.fromaddress;
        // D s_82_2: cast zx s_82_1 -> bv
        let s_82_2: Bits = Bits::new(s_82_1 as u128, 64u16);
        // C s_82_3: const #1s : i64
        let s_82_3: i64 = 1;
        // C s_82_4: cast zx s_82_3 -> i
        let s_82_4: i128 = (i128::try_from(s_82_3).unwrap());
        // C s_82_5: const #55s : i
        let s_82_5: i128 = 55;
        // C s_82_6: add s_82_5 s_82_4
        let s_82_6: i128 = (s_82_5 + s_82_4);
        // D s_82_7: bit-extract s_82_2 s_82_0 s_82_6
        let s_82_7: Bits = (Bits::new(
            ((s_82_2) >> (s_82_0)).value(),
            u16::try_from(s_82_6).unwrap(),
        ));
        // D s_82_8: cast reint s_82_7 -> u56
        let s_82_8: u64 = (s_82_7.value() as u64);
        // D s_82_9: cast zx s_82_8 -> bv
        let s_82_9: Bits = Bits::new(s_82_8 as u128, 56u16);
        // D s_82_10: cast zx s_82_9 -> i
        let s_82_10: i128 = (s_82_9.value() as i128);
        // D s_82_11: cast reint s_82_10 -> i64
        let s_82_11: i64 = (s_82_10 as i64);
        // C s_82_12: const #0s : i
        let s_82_12: i128 = 0;
        // D s_82_13: read-var toaddress:u64
        let s_82_13: u64 = fn_state.toaddress;
        // D s_82_14: cast zx s_82_13 -> bv
        let s_82_14: Bits = Bits::new(s_82_13 as u128, 64u16);
        // C s_82_15: const #1s : i64
        let s_82_15: i64 = 1;
        // C s_82_16: cast zx s_82_15 -> i
        let s_82_16: i128 = (i128::try_from(s_82_15).unwrap());
        // C s_82_17: const #55s : i
        let s_82_17: i128 = 55;
        // C s_82_18: add s_82_17 s_82_16
        let s_82_18: i128 = (s_82_17 + s_82_16);
        // D s_82_19: bit-extract s_82_14 s_82_12 s_82_18
        let s_82_19: Bits = (Bits::new(
            ((s_82_14) >> (s_82_12)).value(),
            u16::try_from(s_82_18).unwrap(),
        ));
        // D s_82_20: cast reint s_82_19 -> u56
        let s_82_20: u64 = (s_82_19.value() as u64);
        // D s_82_21: cast zx s_82_20 -> bv
        let s_82_21: Bits = Bits::new(s_82_20 as u128, 56u16);
        // D s_82_22: cast zx s_82_21 -> i
        let s_82_22: i128 = (s_82_21.value() as i128);
        // D s_82_23: cast reint s_82_22 -> i64
        let s_82_23: i64 = (s_82_22 as i64);
        // D s_82_24: cast zx s_82_11 -> i
        let s_82_24: i128 = (i128::try_from(s_82_11).unwrap());
        // D s_82_25: cast zx s_82_23 -> i
        let s_82_25: i128 = (i128::try_from(s_82_23).unwrap());
        // D s_82_26: cmp-gt s_82_24 s_82_25
        let s_82_26: bool = ((s_82_24) > (s_82_25));
        // N s_82_27: branch s_82_26 b106 b83
        if s_82_26 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #0u : u8
        let s_83_0: bool = false;
        // D s_83_1: write-var gs#146603 <= s_83_0
        fn_state.gs_146603 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#146603:u8
        let s_84_0: bool = fn_state.gs_146603;
        // N s_84_1: branch s_84_0 b105 b85
        if s_84_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #0s : i
        let s_85_0: i128 = 0;
        // D s_85_1: read-var fromaddress:u64
        let s_85_1: u64 = fn_state.fromaddress;
        // D s_85_2: cast zx s_85_1 -> bv
        let s_85_2: Bits = Bits::new(s_85_1 as u128, 64u16);
        // C s_85_3: const #1s : i64
        let s_85_3: i64 = 1;
        // C s_85_4: cast zx s_85_3 -> i
        let s_85_4: i128 = (i128::try_from(s_85_3).unwrap());
        // C s_85_5: const #55s : i
        let s_85_5: i128 = 55;
        // C s_85_6: add s_85_5 s_85_4
        let s_85_6: i128 = (s_85_5 + s_85_4);
        // D s_85_7: bit-extract s_85_2 s_85_0 s_85_6
        let s_85_7: Bits = (Bits::new(
            ((s_85_2) >> (s_85_0)).value(),
            u16::try_from(s_85_6).unwrap(),
        ));
        // D s_85_8: cast reint s_85_7 -> u56
        let s_85_8: u64 = (s_85_7.value() as u64);
        // D s_85_9: cast zx s_85_8 -> bv
        let s_85_9: Bits = Bits::new(s_85_8 as u128, 56u16);
        // D s_85_10: cast zx s_85_9 -> i
        let s_85_10: i128 = (s_85_9.value() as i128);
        // D s_85_11: cast reint s_85_10 -> i64
        let s_85_11: i64 = (s_85_10 as i64);
        // C s_85_12: const #0s : i
        let s_85_12: i128 = 0;
        // D s_85_13: read-var toaddress:u64
        let s_85_13: u64 = fn_state.toaddress;
        // D s_85_14: cast zx s_85_13 -> bv
        let s_85_14: Bits = Bits::new(s_85_13 as u128, 64u16);
        // C s_85_15: const #1s : i64
        let s_85_15: i64 = 1;
        // C s_85_16: cast zx s_85_15 -> i
        let s_85_16: i128 = (i128::try_from(s_85_15).unwrap());
        // C s_85_17: const #55s : i
        let s_85_17: i128 = 55;
        // C s_85_18: add s_85_17 s_85_16
        let s_85_18: i128 = (s_85_17 + s_85_16);
        // D s_85_19: bit-extract s_85_14 s_85_12 s_85_18
        let s_85_19: Bits = (Bits::new(
            ((s_85_14) >> (s_85_12)).value(),
            u16::try_from(s_85_18).unwrap(),
        ));
        // D s_85_20: cast reint s_85_19 -> u56
        let s_85_20: u64 = (s_85_19.value() as u64);
        // D s_85_21: cast zx s_85_20 -> bv
        let s_85_21: Bits = Bits::new(s_85_20 as u128, 56u16);
        // D s_85_22: cast zx s_85_21 -> i
        let s_85_22: i128 = (s_85_21.value() as i128);
        // D s_85_23: cast reint s_85_22 -> i64
        let s_85_23: i64 = (s_85_22 as i64);
        // D s_85_24: cast zx s_85_11 -> i
        let s_85_24: i128 = (i128::try_from(s_85_11).unwrap());
        // D s_85_25: cast zx s_85_23 -> i
        let s_85_25: i128 = (i128::try_from(s_85_23).unwrap());
        // D s_85_26: cmp-lt s_85_24 s_85_25
        let s_85_26: bool = ((s_85_24) < (s_85_25));
        // N s_85_27: branch s_85_26 b104 b86
        if s_85_26 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var gs#146631 <= s_86_0
        fn_state.gs_146631 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#146631:u8
        let s_87_0: bool = fn_state.gs_146631;
        // N s_87_1: branch s_87_0 b103 b88
        if s_87_0 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var fromaddress:u64
        let s_88_0: u64 = fn_state.fromaddress;
        // D s_88_1: read-var toaddress:u64
        let s_88_1: u64 = fn_state.toaddress;
        // D s_88_2: read-var cpysize:u64
        let s_88_2: u64 = fn_state.cpysize;
        // D s_88_3: call MemCpyDirectionChoice(s_88_0, s_88_1, s_88_2)
        let s_88_3: bool = MemCpyDirectionChoice(state, tracer, s_88_0, s_88_1, s_88_2);
        // D s_88_4: write-var forward <= s_88_3
        fn_state.forward = s_88_3;
        // N s_88_5: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var supports_option_a:u8
        let s_89_0: bool = fn_state.supports_option_a;
        // N s_89_1: branch s_89_0 b99 b90
        if s_89_0 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var forward:u8
        let s_90_0: bool = fn_state.forward;
        // D s_90_1: not s_90_0
        let s_90_1: bool = !s_90_0;
        // N s_90_2: branch s_90_1 b98 b91
        if s_90_1 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #2u : u8
        let s_91_0: u8 = 2;
        // D s_91_1: write-var nzcv <= s_91_0
        fn_state.nzcv = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var toaddress:u64
        let s_92_0: u64 = fn_state.toaddress;
        // D s_92_1: read-var fromaddress:u64
        let s_92_1: u64 = fn_state.fromaddress;
        // D s_92_2: read-var cpysize:u64
        let s_92_2: u64 = fn_state.cpysize;
        // D s_92_3: call CPYPreSizeChoice(s_92_0, s_92_1, s_92_2)
        let s_92_3: u64 = CPYPreSizeChoice(state, tracer, s_92_0, s_92_1, s_92_2);
        // D s_92_4: write-var stagecpysize <= s_92_3
        fn_state.stagecpysize = s_92_3;
        // C s_92_5: const #63s : i
        let s_92_5: i128 = 63;
        // D s_92_6: read-var stagecpysize:u64
        let s_92_6: u64 = fn_state.stagecpysize;
        // D s_92_7: cast zx s_92_6 -> bv
        let s_92_7: Bits = Bits::new(s_92_6 as u128, 64u16);
        // C s_92_8: const #1u : u64
        let s_92_8: u64 = 1;
        // D s_92_9: bit-extract s_92_7 s_92_5 s_92_8
        let s_92_9: Bits = (Bits::new(
            ((s_92_7) >> (s_92_5)).value(),
            u16::try_from(s_92_8).unwrap(),
        ));
        // D s_92_10: cast reint s_92_9 -> u8
        let s_92_10: bool = ((s_92_9.value()) != 0);
        // C s_92_11: const #0s : i
        let s_92_11: i128 = 0;
        // C s_92_12: const #0u : u64
        let s_92_12: u64 = 0;
        // D s_92_13: cast zx s_92_10 -> u64
        let s_92_13: u64 = (s_92_10 as u64);
        // C s_92_14: const #1u : u64
        let s_92_14: u64 = 1;
        // D s_92_15: and s_92_13 s_92_14
        let s_92_15: u64 = ((s_92_13) & (s_92_14));
        // D s_92_16: cmp-eq s_92_15 s_92_14
        let s_92_16: bool = ((s_92_15) == (s_92_14));
        // D s_92_17: lsl s_92_13 s_92_11
        let s_92_17: u64 = s_92_13 << s_92_11;
        // D s_92_18: or s_92_12 s_92_17
        let s_92_18: u64 = ((s_92_12) | (s_92_17));
        // D s_92_19: cmpl s_92_17
        let s_92_19: u64 = !s_92_17;
        // D s_92_20: and s_92_12 s_92_19
        let s_92_20: u64 = ((s_92_12) & (s_92_19));
        // D s_92_21: select s_92_16 s_92_18 s_92_20
        let s_92_21: u64 = if s_92_16 { s_92_18 } else { s_92_20 };
        // D s_92_22: cast trunc s_92_21 -> u8
        let s_92_22: bool = ((s_92_21) != 0);
        // C s_92_23: const #63s : i
        let s_92_23: i128 = 63;
        // D s_92_24: read-var cpysize:u64
        let s_92_24: u64 = fn_state.cpysize;
        // D s_92_25: cast zx s_92_24 -> bv
        let s_92_25: Bits = Bits::new(s_92_24 as u128, 64u16);
        // C s_92_26: const #1u : u64
        let s_92_26: u64 = 1;
        // D s_92_27: bit-extract s_92_25 s_92_23 s_92_26
        let s_92_27: Bits = (Bits::new(
            ((s_92_25) >> (s_92_23)).value(),
            u16::try_from(s_92_26).unwrap(),
        ));
        // D s_92_28: cast reint s_92_27 -> u8
        let s_92_28: bool = ((s_92_27.value()) != 0);
        // C s_92_29: const #0s : i
        let s_92_29: i128 = 0;
        // C s_92_30: const #0u : u64
        let s_92_30: u64 = 0;
        // D s_92_31: cast zx s_92_28 -> u64
        let s_92_31: u64 = (s_92_28 as u64);
        // C s_92_32: const #1u : u64
        let s_92_32: u64 = 1;
        // D s_92_33: and s_92_31 s_92_32
        let s_92_33: u64 = ((s_92_31) & (s_92_32));
        // D s_92_34: cmp-eq s_92_33 s_92_32
        let s_92_34: bool = ((s_92_33) == (s_92_32));
        // D s_92_35: lsl s_92_31 s_92_29
        let s_92_35: u64 = s_92_31 << s_92_29;
        // D s_92_36: or s_92_30 s_92_35
        let s_92_36: u64 = ((s_92_30) | (s_92_35));
        // D s_92_37: cmpl s_92_35
        let s_92_37: u64 = !s_92_35;
        // D s_92_38: and s_92_30 s_92_37
        let s_92_38: u64 = ((s_92_30) & (s_92_37));
        // D s_92_39: select s_92_34 s_92_36 s_92_38
        let s_92_39: u64 = if s_92_34 { s_92_36 } else { s_92_38 };
        // D s_92_40: cast trunc s_92_39 -> u8
        let s_92_40: bool = ((s_92_39) != 0);
        // D s_92_41: cast zx s_92_22 -> bv
        let s_92_41: Bits = Bits::new(s_92_22 as u128, 1u16);
        // D s_92_42: cast zx s_92_40 -> bv
        let s_92_42: Bits = Bits::new(s_92_40 as u128, 1u16);
        // D s_92_43: cmp-eq s_92_41 s_92_42
        let s_92_43: bool = ((s_92_41) == (s_92_42));
        // N s_92_44: branch s_92_43 b97 b93
        if s_92_43 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #64s : i
        let s_93_0: i128 = 64;
        // S s_93_1: call Zeros(s_93_0)
        let s_93_1: Bits = Zeros(state, tracer, s_93_0);
        // S s_93_2: cast reint s_93_1 -> u64
        let s_93_2: u64 = (s_93_1.value() as u64);
        // D s_93_3: read-var stagecpysize:u64
        let s_93_3: u64 = fn_state.stagecpysize;
        // D s_93_4: cast zx s_93_3 -> bv
        let s_93_4: Bits = Bits::new(s_93_3 as u128, 64u16);
        // S s_93_5: cast zx s_93_2 -> bv
        let s_93_5: Bits = Bits::new(s_93_2 as u128, 64u16);
        // D s_93_6: cmp-eq s_93_4 s_93_5
        let s_93_6: bool = ((s_93_4) == (s_93_5));
        // D s_93_7: write-var gs#146609 <= s_93_6
        fn_state.gs_146609 = s_93_6;
        // N s_93_8: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#146609:u8
        let s_94_0: bool = fn_state.gs_146609;
        // N s_94_1: assert s_94_0
        let s_94_1: () = assert!(s_94_0);
        // D s_94_2: read-var cpysize:u64
        let s_94_2: u64 = fn_state.cpysize;
        // D s_94_3: cast zx s_94_2 -> bv
        let s_94_3: Bits = Bits::new(s_94_2 as u128, 64u16);
        // D s_94_4: cast sx s_94_3 -> i
        let s_94_4: i128 = {
            let sign_bit = s_94_3.length() - 1;
            let mut result = s_94_3.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_94_5: cast reint s_94_4 -> i64
        let s_94_5: i64 = (s_94_4 as i64);
        // C s_94_6: const #0s : i
        let s_94_6: i128 = 0;
        // D s_94_7: cast zx s_94_5 -> i
        let s_94_7: i128 = (i128::try_from(s_94_5).unwrap());
        // D s_94_8: cmp-gt s_94_7 s_94_6
        let s_94_8: bool = ((s_94_7) > (s_94_6));
        // N s_94_9: branch s_94_8 b96 b95
        if s_94_8 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var stagecpysize:u64
        let s_95_0: u64 = fn_state.stagecpysize;
        // D s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 64u16);
        // D s_95_2: cast sx s_95_1 -> i
        let s_95_2: i128 = {
            let sign_bit = s_95_1.length() - 1;
            let mut result = s_95_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_95_3: cast reint s_95_2 -> i64
        let s_95_3: i64 = (s_95_2 as i64);
        // D s_95_4: read-var cpysize:u64
        let s_95_4: u64 = fn_state.cpysize;
        // D s_95_5: cast zx s_95_4 -> bv
        let s_95_5: Bits = Bits::new(s_95_4 as u128, 64u16);
        // D s_95_6: cast sx s_95_5 -> i
        let s_95_6: i128 = {
            let sign_bit = s_95_5.length() - 1;
            let mut result = s_95_5.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_95_7: cast reint s_95_6 -> i64
        let s_95_7: i64 = (s_95_6 as i64);
        // D s_95_8: cast zx s_95_3 -> i
        let s_95_8: i128 = (i128::try_from(s_95_3).unwrap());
        // D s_95_9: cast zx s_95_7 -> i
        let s_95_9: i128 = (i128::try_from(s_95_7).unwrap());
        // D s_95_10: cmp-ge s_95_8 s_95_9
        let s_95_10: bool = ((s_95_8) >= (s_95_9));
        // N s_95_11: assert s_95_10
        let s_95_11: () = assert!(s_95_10);
        // N s_95_12: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var stagecpysize:u64
        let s_96_0: u64 = fn_state.stagecpysize;
        // D s_96_1: cast zx s_96_0 -> bv
        let s_96_1: Bits = Bits::new(s_96_0 as u128, 64u16);
        // D s_96_2: cast sx s_96_1 -> i
        let s_96_2: i128 = {
            let sign_bit = s_96_1.length() - 1;
            let mut result = s_96_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_96_3: cast reint s_96_2 -> i64
        let s_96_3: i64 = (s_96_2 as i64);
        // D s_96_4: read-var cpysize:u64
        let s_96_4: u64 = fn_state.cpysize;
        // D s_96_5: cast zx s_96_4 -> bv
        let s_96_5: Bits = Bits::new(s_96_4 as u128, 64u16);
        // D s_96_6: cast sx s_96_5 -> i
        let s_96_6: i128 = {
            let sign_bit = s_96_5.length() - 1;
            let mut result = s_96_5.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_96_7: cast reint s_96_6 -> i64
        let s_96_7: i64 = (s_96_6 as i64);
        // D s_96_8: cast zx s_96_3 -> i
        let s_96_8: i128 = (i128::try_from(s_96_3).unwrap());
        // D s_96_9: cast zx s_96_7 -> i
        let s_96_9: i128 = (i128::try_from(s_96_7).unwrap());
        // D s_96_10: cmp-le s_96_8 s_96_9
        let s_96_10: bool = ((s_96_8) <= (s_96_9));
        // N s_96_11: assert s_96_10
        let s_96_11: () = assert!(s_96_10);
        // N s_96_12: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #1u : u8
        let s_97_0: bool = true;
        // D s_97_1: write-var gs#146609 <= s_97_0
        fn_state.gs_146609 = s_97_0;
        // N s_97_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var toaddress:u64
        let s_98_0: u64 = fn_state.toaddress;
        // D s_98_1: cast zx s_98_0 -> bv
        let s_98_1: Bits = Bits::new(s_98_0 as u128, 64u16);
        // D s_98_2: read-var cpysize:u64
        let s_98_2: u64 = fn_state.cpysize;
        // D s_98_3: cast zx s_98_2 -> bv
        let s_98_3: Bits = Bits::new(s_98_2 as u128, 64u16);
        // D s_98_4: add s_98_1 s_98_3
        let s_98_4: Bits = (s_98_1 + s_98_3);
        // D s_98_5: cast reint s_98_4 -> u64
        let s_98_5: u64 = (s_98_4.value() as u64);
        // D s_98_6: write-var toaddress <= s_98_5
        fn_state.toaddress = s_98_5;
        // D s_98_7: read-var fromaddress:u64
        let s_98_7: u64 = fn_state.fromaddress;
        // D s_98_8: cast zx s_98_7 -> bv
        let s_98_8: Bits = Bits::new(s_98_7 as u128, 64u16);
        // D s_98_9: read-var cpysize:u64
        let s_98_9: u64 = fn_state.cpysize;
        // D s_98_10: cast zx s_98_9 -> bv
        let s_98_10: Bits = Bits::new(s_98_9 as u128, 64u16);
        // D s_98_11: add s_98_8 s_98_10
        let s_98_11: Bits = (s_98_8 + s_98_10);
        // D s_98_12: cast reint s_98_11 -> u64
        let s_98_12: u64 = (s_98_11.value() as u64);
        // D s_98_13: write-var fromaddress <= s_98_12
        fn_state.fromaddress = s_98_12;
        // C s_98_14: const #10u : u8
        let s_98_14: u8 = 10;
        // D s_98_15: write-var nzcv <= s_98_14
        fn_state.nzcv = s_98_14;
        // N s_98_16: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #0u : u8
        let s_99_0: u8 = 0;
        // D s_99_1: write-var nzcv <= s_99_0
        fn_state.nzcv = s_99_0;
        // D s_99_2: read-var forward:u8
        let s_99_2: bool = fn_state.forward;
        // N s_99_3: branch s_99_2 b102 b100
        if s_99_2 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_100_0: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_101_0: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var toaddress:u64
        let s_102_0: u64 = fn_state.toaddress;
        // D s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 64u16);
        // D s_102_2: read-var cpysize:u64
        let s_102_2: u64 = fn_state.cpysize;
        // D s_102_3: cast zx s_102_2 -> bv
        let s_102_3: Bits = Bits::new(s_102_2 as u128, 64u16);
        // D s_102_4: add s_102_1 s_102_3
        let s_102_4: Bits = (s_102_1 + s_102_3);
        // D s_102_5: cast reint s_102_4 -> u64
        let s_102_5: u64 = (s_102_4.value() as u64);
        // D s_102_6: write-var toaddress <= s_102_5
        fn_state.toaddress = s_102_5;
        // D s_102_7: read-var fromaddress:u64
        let s_102_7: u64 = fn_state.fromaddress;
        // D s_102_8: cast zx s_102_7 -> bv
        let s_102_8: Bits = Bits::new(s_102_7 as u128, 64u16);
        // D s_102_9: read-var cpysize:u64
        let s_102_9: u64 = fn_state.cpysize;
        // D s_102_10: cast zx s_102_9 -> bv
        let s_102_10: Bits = Bits::new(s_102_9 as u128, 64u16);
        // D s_102_11: add s_102_8 s_102_10
        let s_102_11: Bits = (s_102_8 + s_102_10);
        // D s_102_12: cast reint s_102_11 -> u64
        let s_102_12: u64 = (s_102_11.value() as u64);
        // D s_102_13: write-var fromaddress <= s_102_12
        fn_state.fromaddress = s_102_12;
        // C s_102_14: const #64s : i
        let s_102_14: i128 = 64;
        // S s_102_15: call Zeros(s_102_14)
        let s_102_15: Bits = Zeros(state, tracer, s_102_14);
        // S s_102_16: cast reint s_102_15 -> u64
        let s_102_16: u64 = (s_102_15.value() as u64);
        // S s_102_17: cast zx s_102_16 -> bv
        let s_102_17: Bits = Bits::new(s_102_16 as u128, 64u16);
        // D s_102_18: read-var cpysize:u64
        let s_102_18: u64 = fn_state.cpysize;
        // D s_102_19: cast zx s_102_18 -> bv
        let s_102_19: Bits = Bits::new(s_102_18 as u128, 64u16);
        // D s_102_20: sub s_102_17 s_102_19
        let s_102_20: Bits = ((s_102_17) - (s_102_19));
        // D s_102_21: cast reint s_102_20 -> u64
        let s_102_21: u64 = (s_102_20.value() as u64);
        // D s_102_22: write-var cpysize <= s_102_21
        fn_state.cpysize = s_102_21;
        // N s_102_23: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #0u : u8
        let s_103_0: bool = false;
        // D s_103_1: write-var forward <= s_103_0
        fn_state.forward = s_103_0;
        // N s_103_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #0s : i
        let s_104_0: i128 = 0;
        // D s_104_1: read-var fromaddress:u64
        let s_104_1: u64 = fn_state.fromaddress;
        // D s_104_2: cast zx s_104_1 -> bv
        let s_104_2: Bits = Bits::new(s_104_1 as u128, 64u16);
        // C s_104_3: const #1s : i64
        let s_104_3: i64 = 1;
        // C s_104_4: cast zx s_104_3 -> i
        let s_104_4: i128 = (i128::try_from(s_104_3).unwrap());
        // C s_104_5: const #55s : i
        let s_104_5: i128 = 55;
        // C s_104_6: add s_104_5 s_104_4
        let s_104_6: i128 = (s_104_5 + s_104_4);
        // D s_104_7: bit-extract s_104_2 s_104_0 s_104_6
        let s_104_7: Bits = (Bits::new(
            ((s_104_2) >> (s_104_0)).value(),
            u16::try_from(s_104_6).unwrap(),
        ));
        // D s_104_8: cast reint s_104_7 -> u56
        let s_104_8: u64 = (s_104_7.value() as u64);
        // C s_104_9: const #0s : i
        let s_104_9: i128 = 0;
        // D s_104_10: read-var cpysize:u64
        let s_104_10: u64 = fn_state.cpysize;
        // D s_104_11: cast zx s_104_10 -> bv
        let s_104_11: Bits = Bits::new(s_104_10 as u128, 64u16);
        // C s_104_12: const #1s : i64
        let s_104_12: i64 = 1;
        // C s_104_13: cast zx s_104_12 -> i
        let s_104_13: i128 = (i128::try_from(s_104_12).unwrap());
        // C s_104_14: const #55s : i
        let s_104_14: i128 = 55;
        // C s_104_15: add s_104_14 s_104_13
        let s_104_15: i128 = (s_104_14 + s_104_13);
        // D s_104_16: bit-extract s_104_11 s_104_9 s_104_15
        let s_104_16: Bits = (Bits::new(
            ((s_104_11) >> (s_104_9)).value(),
            u16::try_from(s_104_15).unwrap(),
        ));
        // D s_104_17: cast reint s_104_16 -> u56
        let s_104_17: u64 = (s_104_16.value() as u64);
        // D s_104_18: cast zx s_104_8 -> bv
        let s_104_18: Bits = Bits::new(s_104_8 as u128, 56u16);
        // D s_104_19: cast zx s_104_17 -> bv
        let s_104_19: Bits = Bits::new(s_104_17 as u128, 56u16);
        // D s_104_20: add s_104_18 s_104_19
        let s_104_20: Bits = (s_104_18 + s_104_19);
        // D s_104_21: cast reint s_104_20 -> u56
        let s_104_21: u64 = (s_104_20.value() as u64);
        // D s_104_22: cast zx s_104_21 -> bv
        let s_104_22: Bits = Bits::new(s_104_21 as u128, 56u16);
        // D s_104_23: cast zx s_104_22 -> i
        let s_104_23: i128 = (s_104_22.value() as i128);
        // D s_104_24: cast reint s_104_23 -> i64
        let s_104_24: i64 = (s_104_23 as i64);
        // C s_104_25: const #0s : i
        let s_104_25: i128 = 0;
        // D s_104_26: read-var toaddress:u64
        let s_104_26: u64 = fn_state.toaddress;
        // D s_104_27: cast zx s_104_26 -> bv
        let s_104_27: Bits = Bits::new(s_104_26 as u128, 64u16);
        // C s_104_28: const #1s : i64
        let s_104_28: i64 = 1;
        // C s_104_29: cast zx s_104_28 -> i
        let s_104_29: i128 = (i128::try_from(s_104_28).unwrap());
        // C s_104_30: const #55s : i
        let s_104_30: i128 = 55;
        // C s_104_31: add s_104_30 s_104_29
        let s_104_31: i128 = (s_104_30 + s_104_29);
        // D s_104_32: bit-extract s_104_27 s_104_25 s_104_31
        let s_104_32: Bits = (Bits::new(
            ((s_104_27) >> (s_104_25)).value(),
            u16::try_from(s_104_31).unwrap(),
        ));
        // D s_104_33: cast reint s_104_32 -> u56
        let s_104_33: u64 = (s_104_32.value() as u64);
        // D s_104_34: cast zx s_104_33 -> bv
        let s_104_34: Bits = Bits::new(s_104_33 as u128, 56u16);
        // D s_104_35: cast zx s_104_34 -> i
        let s_104_35: i128 = (s_104_34.value() as i128);
        // D s_104_36: cast reint s_104_35 -> i64
        let s_104_36: i64 = (s_104_35 as i64);
        // D s_104_37: cast zx s_104_24 -> i
        let s_104_37: i128 = (i128::try_from(s_104_24).unwrap());
        // D s_104_38: cast zx s_104_36 -> i
        let s_104_38: i128 = (i128::try_from(s_104_36).unwrap());
        // D s_104_39: cmp-gt s_104_37 s_104_38
        let s_104_39: bool = ((s_104_37) > (s_104_38));
        // D s_104_40: write-var gs#146631 <= s_104_39
        fn_state.gs_146631 = s_104_39;
        // N s_104_41: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #1u : u8
        let s_105_0: bool = true;
        // D s_105_1: write-var forward <= s_105_0
        fn_state.forward = s_105_0;
        // N s_105_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #0s : i
        let s_106_0: i128 = 0;
        // D s_106_1: read-var fromaddress:u64
        let s_106_1: u64 = fn_state.fromaddress;
        // D s_106_2: cast zx s_106_1 -> bv
        let s_106_2: Bits = Bits::new(s_106_1 as u128, 64u16);
        // C s_106_3: const #1s : i64
        let s_106_3: i64 = 1;
        // C s_106_4: cast zx s_106_3 -> i
        let s_106_4: i128 = (i128::try_from(s_106_3).unwrap());
        // C s_106_5: const #55s : i
        let s_106_5: i128 = 55;
        // C s_106_6: add s_106_5 s_106_4
        let s_106_6: i128 = (s_106_5 + s_106_4);
        // D s_106_7: bit-extract s_106_2 s_106_0 s_106_6
        let s_106_7: Bits = (Bits::new(
            ((s_106_2) >> (s_106_0)).value(),
            u16::try_from(s_106_6).unwrap(),
        ));
        // D s_106_8: cast reint s_106_7 -> u56
        let s_106_8: u64 = (s_106_7.value() as u64);
        // D s_106_9: cast zx s_106_8 -> bv
        let s_106_9: Bits = Bits::new(s_106_8 as u128, 56u16);
        // D s_106_10: cast zx s_106_9 -> i
        let s_106_10: i128 = (s_106_9.value() as i128);
        // D s_106_11: cast reint s_106_10 -> i64
        let s_106_11: i64 = (s_106_10 as i64);
        // C s_106_12: const #0s : i
        let s_106_12: i128 = 0;
        // D s_106_13: read-var toaddress:u64
        let s_106_13: u64 = fn_state.toaddress;
        // D s_106_14: cast zx s_106_13 -> bv
        let s_106_14: Bits = Bits::new(s_106_13 as u128, 64u16);
        // C s_106_15: const #1s : i64
        let s_106_15: i64 = 1;
        // C s_106_16: cast zx s_106_15 -> i
        let s_106_16: i128 = (i128::try_from(s_106_15).unwrap());
        // C s_106_17: const #55s : i
        let s_106_17: i128 = 55;
        // C s_106_18: add s_106_17 s_106_16
        let s_106_18: i128 = (s_106_17 + s_106_16);
        // D s_106_19: bit-extract s_106_14 s_106_12 s_106_18
        let s_106_19: Bits = (Bits::new(
            ((s_106_14) >> (s_106_12)).value(),
            u16::try_from(s_106_18).unwrap(),
        ));
        // D s_106_20: cast reint s_106_19 -> u56
        let s_106_20: u64 = (s_106_19.value() as u64);
        // C s_106_21: const #0s : i
        let s_106_21: i128 = 0;
        // D s_106_22: read-var cpysize:u64
        let s_106_22: u64 = fn_state.cpysize;
        // D s_106_23: cast zx s_106_22 -> bv
        let s_106_23: Bits = Bits::new(s_106_22 as u128, 64u16);
        // C s_106_24: const #1s : i64
        let s_106_24: i64 = 1;
        // C s_106_25: cast zx s_106_24 -> i
        let s_106_25: i128 = (i128::try_from(s_106_24).unwrap());
        // C s_106_26: const #55s : i
        let s_106_26: i128 = 55;
        // C s_106_27: add s_106_26 s_106_25
        let s_106_27: i128 = (s_106_26 + s_106_25);
        // D s_106_28: bit-extract s_106_23 s_106_21 s_106_27
        let s_106_28: Bits = (Bits::new(
            ((s_106_23) >> (s_106_21)).value(),
            u16::try_from(s_106_27).unwrap(),
        ));
        // D s_106_29: cast reint s_106_28 -> u56
        let s_106_29: u64 = (s_106_28.value() as u64);
        // D s_106_30: cast zx s_106_20 -> bv
        let s_106_30: Bits = Bits::new(s_106_20 as u128, 56u16);
        // D s_106_31: cast zx s_106_29 -> bv
        let s_106_31: Bits = Bits::new(s_106_29 as u128, 56u16);
        // D s_106_32: add s_106_30 s_106_31
        let s_106_32: Bits = (s_106_30 + s_106_31);
        // D s_106_33: cast reint s_106_32 -> u56
        let s_106_33: u64 = (s_106_32.value() as u64);
        // D s_106_34: cast zx s_106_33 -> bv
        let s_106_34: Bits = Bits::new(s_106_33 as u128, 56u16);
        // D s_106_35: cast zx s_106_34 -> i
        let s_106_35: i128 = (s_106_34.value() as i128);
        // D s_106_36: cast reint s_106_35 -> i64
        let s_106_36: i64 = (s_106_35 as i64);
        // D s_106_37: cast zx s_106_11 -> i
        let s_106_37: i128 = (i128::try_from(s_106_11).unwrap());
        // D s_106_38: cast zx s_106_36 -> i
        let s_106_38: i128 = (i128::try_from(s_106_36).unwrap());
        // D s_106_39: cmp-lt s_106_37 s_106_38
        let s_106_39: bool = ((s_106_37) < (s_106_38));
        // D s_106_40: write-var gs#146603 <= s_106_39
        fn_state.gs_146603 = s_106_39;
        // N s_106_41: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #36028797018963967u : u64
        let s_107_0: u64 = 36028797018963967;
        // D s_107_1: write-var cpysize <= s_107_0
        fn_state.cpysize = s_107_0;
        // N s_107_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #() : ()
        let s_108_0: () = ();
        // S s_108_1: call AArch64_IsUnprivAccessPriv(s_108_0)
        let s_108_1: bool = AArch64_IsUnprivAccessPriv(state, tracer, s_108_0);
        // D s_108_2: write-var wprivileged <= s_108_1
        fn_state.wprivileged = s_108_1;
        // N s_108_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #() : ()
        let s_109_0: () = ();
        // S s_109_1: call AArch64_IsUnprivAccessPriv(s_109_0)
        let s_109_1: bool = AArch64_IsUnprivAccessPriv(state, tracer, s_109_0);
        // D s_109_2: write-var rprivileged <= s_109_1
        fn_state.rprivileged = s_109_1;
        // N s_109_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
