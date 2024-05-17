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
use CreateAccDescMOPS::*;
use SPESampleMemSet::*;
use SETPreSizeChoice::*;
use X_read::*;
use Zeros::*;
use SETSizeChoice::*;
use Mem_set::*;
use neq_int::*;
use X_set::*;
use SETOptionA::*;
use u__id::*;
use MemSetZeroSizeCheck::*;
use replicate_bits_borealis_internal::*;
use SETPostSizeChoice::*;
use MemSetParametersIllformedM::*;
use MemSetParametersIllformedE::*;
use MismatchedMemSetException::*;
use AArch64_IsUnprivAccessPriv::*;
use common::*;
pub fn execute_aarch64_instrs_memory_mcpymset_set<T: Tracer>(
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
        gs_167876: bool,
        toaddress: u64,
        B: i128,
        postsize: u64,
        gs_167888: bool,
        nzcv: u8,
        setsize: u64,
        gs_167865: bool,
        supports_option_a: bool,
        gs_167868: bool,
        data: u8,
        privileged: bool,
        accdesc: ProductType9878976b5bcce9c9,
        stagesetsize: u64,
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
        // C s_0_0: const #64s : i64
        let s_0_0: i64 = 64;
        // D s_0_1: read-var d:i64
        let s_0_1: i64 = fn_state.d;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: call X_read(s_0_2, s_0_0)
        let s_0_3: Bits = X_read(state, tracer, s_0_2, s_0_0);
        // D s_0_4: cast reint s_0_3 -> u64
        let s_0_4: u64 = (s_0_3.value() as u64);
        // D s_0_5: write-var toaddress <= s_0_4
        fn_state.toaddress = s_0_4;
        // C s_0_6: const #64s : i64
        let s_0_6: i64 = 64;
        // D s_0_7: read-var n:i64
        let s_0_7: i64 = fn_state.n;
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_9: call X_read(s_0_8, s_0_6)
        let s_0_9: Bits = X_read(state, tracer, s_0_8, s_0_6);
        // D s_0_10: cast reint s_0_9 -> u64
        let s_0_10: u64 = (s_0_9.value() as u64);
        // D s_0_11: write-var setsize <= s_0_10
        fn_state.setsize = s_0_10;
        // C s_0_12: const #8s : i64
        let s_0_12: i64 = 8;
        // D s_0_13: read-var s:i64
        let s_0_13: i64 = fn_state.s;
        // D s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_15: call X_read(s_0_14, s_0_12)
        let s_0_15: Bits = X_read(state, tracer, s_0_14, s_0_12);
        // D s_0_16: cast reint s_0_15 -> u8
        let s_0_16: u8 = (s_0_15.value() as u8);
        // D s_0_17: write-var data <= s_0_16
        fn_state.data = s_0_16;
        // C s_0_18: const #16984u : u32
        let s_0_18: u32 = 16984;
        // D s_0_19: read-reg s_0_18:u8
        let s_0_19: bool = {
            let value = state.read_register::<bool>(s_0_18 as isize);
            tracer.read_register(s_0_18 as isize, value);
            value
        };
        // C s_0_20: const #16997u : u32
        let s_0_20: u32 = 16997;
        // D s_0_21: read-reg s_0_20:u8
        let s_0_21: bool = {
            let value = state.read_register::<bool>(s_0_20 as isize);
            tracer.read_register(s_0_20 as isize, value);
            value
        };
        // C s_0_22: const #16971u : u32
        let s_0_22: u32 = 16971;
        // D s_0_23: read-reg s_0_22:u8
        let s_0_23: bool = {
            let value = state.read_register::<bool>(s_0_22 as isize);
            tracer.read_register(s_0_22 as isize, value);
            value
        };
        // C s_0_24: const #16996u : u32
        let s_0_24: u32 = 16996;
        // D s_0_25: read-reg s_0_24:u8
        let s_0_25: bool = {
            let value = state.read_register::<bool>(s_0_24 as isize);
            tracer.read_register(s_0_24 as isize, value);
            value
        };
        // D s_0_26: cast zx s_0_23 -> bv
        let s_0_26: Bits = Bits::new(s_0_23 as u128, 1u16);
        // D s_0_27: cast zx s_0_25 -> bv
        let s_0_27: Bits = Bits::new(s_0_25 as u128, 1u16);
        // D s_0_28: cast reint s_0_26 -> u128
        let s_0_28: u128 = (s_0_26.value() as u128);
        // D s_0_29: size-of s_0_26
        let s_0_29: u16 = s_0_26.length();
        // D s_0_30: cast reint s_0_27 -> u128
        let s_0_30: u128 = (s_0_27.value() as u128);
        // D s_0_31: size-of s_0_27
        let s_0_31: u16 = s_0_27.length();
        // D s_0_32: lsl s_0_28 s_0_31
        let s_0_32: u128 = s_0_28 << s_0_31;
        // D s_0_33: or s_0_32 s_0_30
        let s_0_33: u128 = ((s_0_32) | (s_0_30));
        // D s_0_34: add s_0_29 s_0_31
        let s_0_34: u16 = (s_0_29 + s_0_31);
        // D s_0_35: create-bits s_0_33 s_0_34
        let s_0_35: Bits = Bits::new(s_0_33, s_0_34);
        // D s_0_36: cast reint s_0_35 -> u8
        let s_0_36: u8 = (s_0_35.value() as u8);
        // D s_0_37: cast zx s_0_21 -> bv
        let s_0_37: Bits = Bits::new(s_0_21 as u128, 1u16);
        // D s_0_38: cast zx s_0_36 -> bv
        let s_0_38: Bits = Bits::new(s_0_36 as u128, 2u16);
        // D s_0_39: cast reint s_0_37 -> u128
        let s_0_39: u128 = (s_0_37.value() as u128);
        // D s_0_40: size-of s_0_37
        let s_0_40: u16 = s_0_37.length();
        // D s_0_41: cast reint s_0_38 -> u128
        let s_0_41: u128 = (s_0_38.value() as u128);
        // D s_0_42: size-of s_0_38
        let s_0_42: u16 = s_0_38.length();
        // D s_0_43: lsl s_0_39 s_0_42
        let s_0_43: u128 = s_0_39 << s_0_42;
        // D s_0_44: or s_0_43 s_0_41
        let s_0_44: u128 = ((s_0_43) | (s_0_41));
        // D s_0_45: add s_0_40 s_0_42
        let s_0_45: u16 = (s_0_40 + s_0_42);
        // D s_0_46: create-bits s_0_44 s_0_45
        let s_0_46: Bits = Bits::new(s_0_44, s_0_45);
        // D s_0_47: cast reint s_0_46 -> u8
        let s_0_47: u8 = (s_0_46.value() as u8);
        // D s_0_48: cast zx s_0_19 -> bv
        let s_0_48: Bits = Bits::new(s_0_19 as u128, 1u16);
        // D s_0_49: cast zx s_0_47 -> bv
        let s_0_49: Bits = Bits::new(s_0_47 as u128, 3u16);
        // D s_0_50: cast reint s_0_48 -> u128
        let s_0_50: u128 = (s_0_48.value() as u128);
        // D s_0_51: size-of s_0_48
        let s_0_51: u16 = s_0_48.length();
        // D s_0_52: cast reint s_0_49 -> u128
        let s_0_52: u128 = (s_0_49.value() as u128);
        // D s_0_53: size-of s_0_49
        let s_0_53: u16 = s_0_49.length();
        // D s_0_54: lsl s_0_50 s_0_53
        let s_0_54: u128 = s_0_50 << s_0_53;
        // D s_0_55: or s_0_54 s_0_52
        let s_0_55: u128 = ((s_0_54) | (s_0_52));
        // D s_0_56: add s_0_51 s_0_53
        let s_0_56: u16 = (s_0_51 + s_0_53);
        // D s_0_57: create-bits s_0_55 s_0_56
        let s_0_57: Bits = Bits::new(s_0_55, s_0_56);
        // D s_0_58: cast reint s_0_57 -> u8
        let s_0_58: u8 = (s_0_57.value() as u8);
        // D s_0_59: write-var nzcv <= s_0_58
        fn_state.nzcv = s_0_58;
        // C s_0_60: const #() : ()
        let s_0_60: () = ();
        // S s_0_61: call SETOptionA(s_0_60)
        let s_0_61: bool = SETOptionA(state, tracer, s_0_60);
        // D s_0_62: write-var supports_option_a <= s_0_61
        fn_state.supports_option_a = s_0_61;
        // C s_0_63: const #0s : i
        let s_0_63: i128 = 0;
        // D s_0_64: read-var options_name:u8
        let s_0_64: u8 = fn_state.options_name;
        // D s_0_65: cast zx s_0_64 -> bv
        let s_0_65: Bits = Bits::new(s_0_64 as u128, 2u16);
        // C s_0_66: const #1u : u64
        let s_0_66: u64 = 1;
        // D s_0_67: bit-extract s_0_65 s_0_63 s_0_66
        let s_0_67: Bits = (Bits::new(
            ((s_0_65) >> (s_0_63)).value(),
            u16::try_from(s_0_66).unwrap(),
        ));
        // D s_0_68: cast reint s_0_67 -> u8
        let s_0_68: bool = ((s_0_67.value()) != 0);
        // C s_0_69: const #0s : i
        let s_0_69: i128 = 0;
        // C s_0_70: const #0u : u64
        let s_0_70: u64 = 0;
        // D s_0_71: cast zx s_0_68 -> u64
        let s_0_71: u64 = (s_0_68 as u64);
        // C s_0_72: const #1u : u64
        let s_0_72: u64 = 1;
        // D s_0_73: and s_0_71 s_0_72
        let s_0_73: u64 = ((s_0_71) & (s_0_72));
        // D s_0_74: cmp-eq s_0_73 s_0_72
        let s_0_74: bool = ((s_0_73) == (s_0_72));
        // D s_0_75: lsl s_0_71 s_0_69
        let s_0_75: u64 = s_0_71 << s_0_69;
        // D s_0_76: or s_0_70 s_0_75
        let s_0_76: u64 = ((s_0_70) | (s_0_75));
        // D s_0_77: cmpl s_0_75
        let s_0_77: u64 = !s_0_75;
        // D s_0_78: and s_0_70 s_0_77
        let s_0_78: u64 = ((s_0_70) & (s_0_77));
        // D s_0_79: select s_0_74 s_0_76 s_0_78
        let s_0_79: u64 = if s_0_74 { s_0_76 } else { s_0_78 };
        // D s_0_80: cast trunc s_0_79 -> u8
        let s_0_80: bool = ((s_0_79) != 0);
        // D s_0_81: cast zx s_0_80 -> bv
        let s_0_81: Bits = Bits::new(s_0_80 as u128, 1u16);
        // C s_0_82: const #1u : u8
        let s_0_82: bool = true;
        // C s_0_83: cast zx s_0_82 -> bv
        let s_0_83: Bits = Bits::new(s_0_82 as u128, 1u16);
        // D s_0_84: cmp-eq s_0_81 s_0_83
        let s_0_84: bool = ((s_0_81) == (s_0_83));
        // N s_0_85: branch s_0_84 b67 b1
        if s_0_84 {
            return block_67(state, tracer, fn_state);
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
        // D s_1_7: write-var privileged <= s_1_6
        fn_state.privileged = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #1s : i
        let s_2_0: i128 = 1;
        // D s_2_1: read-var options_name:u8
        let s_2_1: u8 = fn_state.options_name;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 2u16);
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
        // C s_2_22: const #1u : u32
        let s_2_22: u32 = 1;
        // D s_2_23: read-var privileged:u8
        let s_2_23: bool = fn_state.privileged;
        // D s_2_24: call CreateAccDescMOPS(s_2_22, s_2_23, s_2_21)
        let s_2_24: ProductType9878976b5bcce9c9 = CreateAccDescMOPS(
            state,
            tracer,
            s_2_22,
            s_2_23,
            s_2_21,
        );
        // D s_2_25: write-var accdesc <= s_2_24
        fn_state.accdesc = s_2_24;
        // D s_2_26: read-var stage:u32
        let s_2_26: u32 = fn_state.stage;
        // C s_2_27: const #0u : u32
        let s_2_27: u32 = 0;
        // D s_2_28: cmp-eq s_2_26 s_2_27
        let s_2_28: bool = ((s_2_26) == (s_2_27));
        // N s_2_29: branch s_2_28 b55 b3
        if s_2_28 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var toaddress:u64
        let s_3_0: u64 = fn_state.toaddress;
        // D s_3_1: read-var setsize:u64
        let s_3_1: u64 = fn_state.setsize;
        // C s_3_2: const #0u : u8
        let s_3_2: bool = false;
        // D s_3_3: call SETPostSizeChoice(s_3_0, s_3_1, s_3_2)
        let s_3_3: u64 = SETPostSizeChoice(state, tracer, s_3_0, s_3_1, s_3_2);
        // D s_3_4: write-var postsize <= s_3_3
        fn_state.postsize = s_3_3;
        // C s_3_5: const #63s : i
        let s_3_5: i128 = 63;
        // D s_3_6: read-var postsize:u64
        let s_3_6: u64 = fn_state.postsize;
        // D s_3_7: cast zx s_3_6 -> bv
        let s_3_7: Bits = Bits::new(s_3_6 as u128, 64u16);
        // C s_3_8: const #1u : u64
        let s_3_8: u64 = 1;
        // D s_3_9: bit-extract s_3_7 s_3_5 s_3_8
        let s_3_9: Bits = (Bits::new(
            ((s_3_7) >> (s_3_5)).value(),
            u16::try_from(s_3_8).unwrap(),
        ));
        // D s_3_10: cast reint s_3_9 -> u8
        let s_3_10: bool = ((s_3_9.value()) != 0);
        // C s_3_11: const #0s : i
        let s_3_11: i128 = 0;
        // C s_3_12: const #0u : u64
        let s_3_12: u64 = 0;
        // D s_3_13: cast zx s_3_10 -> u64
        let s_3_13: u64 = (s_3_10 as u64);
        // C s_3_14: const #1u : u64
        let s_3_14: u64 = 1;
        // D s_3_15: and s_3_13 s_3_14
        let s_3_15: u64 = ((s_3_13) & (s_3_14));
        // D s_3_16: cmp-eq s_3_15 s_3_14
        let s_3_16: bool = ((s_3_15) == (s_3_14));
        // D s_3_17: lsl s_3_13 s_3_11
        let s_3_17: u64 = s_3_13 << s_3_11;
        // D s_3_18: or s_3_12 s_3_17
        let s_3_18: u64 = ((s_3_12) | (s_3_17));
        // D s_3_19: cmpl s_3_17
        let s_3_19: u64 = !s_3_17;
        // D s_3_20: and s_3_12 s_3_19
        let s_3_20: u64 = ((s_3_12) & (s_3_19));
        // D s_3_21: select s_3_16 s_3_18 s_3_20
        let s_3_21: u64 = if s_3_16 { s_3_18 } else { s_3_20 };
        // D s_3_22: cast trunc s_3_21 -> u8
        let s_3_22: bool = ((s_3_21) != 0);
        // C s_3_23: const #63s : i
        let s_3_23: i128 = 63;
        // D s_3_24: read-var setsize:u64
        let s_3_24: u64 = fn_state.setsize;
        // D s_3_25: cast zx s_3_24 -> bv
        let s_3_25: Bits = Bits::new(s_3_24 as u128, 64u16);
        // C s_3_26: const #1u : u64
        let s_3_26: u64 = 1;
        // D s_3_27: bit-extract s_3_25 s_3_23 s_3_26
        let s_3_27: Bits = (Bits::new(
            ((s_3_25) >> (s_3_23)).value(),
            u16::try_from(s_3_26).unwrap(),
        ));
        // D s_3_28: cast reint s_3_27 -> u8
        let s_3_28: bool = ((s_3_27.value()) != 0);
        // C s_3_29: const #0s : i
        let s_3_29: i128 = 0;
        // C s_3_30: const #0u : u64
        let s_3_30: u64 = 0;
        // D s_3_31: cast zx s_3_28 -> u64
        let s_3_31: u64 = (s_3_28 as u64);
        // C s_3_32: const #1u : u64
        let s_3_32: u64 = 1;
        // D s_3_33: and s_3_31 s_3_32
        let s_3_33: u64 = ((s_3_31) & (s_3_32));
        // D s_3_34: cmp-eq s_3_33 s_3_32
        let s_3_34: bool = ((s_3_33) == (s_3_32));
        // D s_3_35: lsl s_3_31 s_3_29
        let s_3_35: u64 = s_3_31 << s_3_29;
        // D s_3_36: or s_3_30 s_3_35
        let s_3_36: u64 = ((s_3_30) | (s_3_35));
        // D s_3_37: cmpl s_3_35
        let s_3_37: u64 = !s_3_35;
        // D s_3_38: and s_3_30 s_3_37
        let s_3_38: u64 = ((s_3_30) & (s_3_37));
        // D s_3_39: select s_3_34 s_3_36 s_3_38
        let s_3_39: u64 = if s_3_34 { s_3_36 } else { s_3_38 };
        // D s_3_40: cast trunc s_3_39 -> u8
        let s_3_40: bool = ((s_3_39) != 0);
        // D s_3_41: cast zx s_3_22 -> bv
        let s_3_41: Bits = Bits::new(s_3_22 as u128, 1u16);
        // D s_3_42: cast zx s_3_40 -> bv
        let s_3_42: Bits = Bits::new(s_3_40 as u128, 1u16);
        // D s_3_43: cmp-eq s_3_41 s_3_42
        let s_3_43: bool = ((s_3_41) == (s_3_42));
        // N s_3_44: branch s_3_43 b54 b4
        if s_3_43 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #64s : i
        let s_4_0: i128 = 64;
        // S s_4_1: call Zeros(s_4_0)
        let s_4_1: Bits = Zeros(state, tracer, s_4_0);
        // S s_4_2: cast reint s_4_1 -> u64
        let s_4_2: u64 = (s_4_1.value() as u64);
        // D s_4_3: read-var postsize:u64
        let s_4_3: u64 = fn_state.postsize;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 64u16);
        // S s_4_5: cast zx s_4_2 -> bv
        let s_4_5: Bits = Bits::new(s_4_2 as u128, 64u16);
        // D s_4_6: cmp-eq s_4_4 s_4_5
        let s_4_6: bool = ((s_4_4) == (s_4_5));
        // D s_4_7: write-var gs#167865 <= s_4_6
        fn_state.gs_167865 = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#167865:u8
        let s_5_0: bool = fn_state.gs_167865;
        // N s_5_1: assert s_5_0
        let s_5_1: () = assert!(s_5_0);
        // C s_5_2: const #() : ()
        let s_5_2: () = ();
        // S s_5_3: call MemSetZeroSizeCheck(s_5_2)
        let s_5_3: bool = MemSetZeroSizeCheck(state, tracer, s_5_2);
        // N s_5_4: branch s_5_3 b53 b6
        if s_5_3 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var setsize:u64
        let s_6_0: u64 = fn_state.setsize;
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
        // D s_6_7: write-var gs#167868 <= s_6_6
        fn_state.gs_167868 = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#167868:u8
        let s_7_0: bool = fn_state.gs_167868;
        // N s_7_1: branch s_7_0 b44 b8
        if s_7_0 {
            return block_44(state, tracer, fn_state);
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
        // D s_9_0: read-var stage:u32
        let s_9_0: u32 = fn_state.stage;
        // C s_9_1: const #1u : u32
        let s_9_1: u32 = 1;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // N s_9_3: branch s_9_2 b40 b10
        if s_9_2 {
            return block_40(state, tracer, fn_state);
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
        // D s_10_1: write-var stagesetsize <= s_10_0
        fn_state.stagesetsize = s_10_0;
        // D s_10_2: read-var setsize:u64
        let s_10_2: u64 = fn_state.setsize;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 64u16);
        // D s_10_4: read-var postsize:u64
        let s_10_4: u64 = fn_state.postsize;
        // D s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 64u16);
        // D s_10_6: cmp-ne s_10_3 s_10_5
        let s_10_6: bool = ((s_10_3) != (s_10_5));
        // N s_10_7: branch s_10_6 b39 b11
        if s_10_6 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var toaddress:u64
        let s_11_0: u64 = fn_state.toaddress;
        // D s_11_1: read-var setsize:u64
        let s_11_1: u64 = fn_state.setsize;
        // C s_11_2: const #0u : u8
        let s_11_2: bool = false;
        // D s_11_3: call MemSetParametersIllformedE(s_11_0, s_11_1, s_11_2)
        let s_11_3: bool = MemSetParametersIllformedE(
            state,
            tracer,
            s_11_0,
            s_11_1,
            s_11_2,
        );
        // D s_11_4: write-var gs#167876 <= s_11_3
        fn_state.gs_167876 = s_11_3;
        // N s_11_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#167876:u8
        let s_12_0: bool = fn_state.gs_167876;
        // N s_12_1: branch s_12_0 b38 b13
        if s_12_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #22416u : u32
        let s_15_0: u32 = 22416;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: bool = {
            let value = state.read_register::<bool>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // N s_15_2: branch s_15_1 b37 b16
        if s_15_1 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
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
        // D s_17_0: read-var supports_option_a:u8
        let s_17_0: bool = fn_state.supports_option_a;
        // N s_17_1: branch s_17_0 b29 b18
        if s_17_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_19_0: read-var stagesetsize:u64
        let s_19_0: u64 = fn_state.stagesetsize;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 64u16);
        // D s_19_2: cast zx s_19_1 -> i
        let s_19_2: i128 = (s_19_1.value() as i128);
        // C s_19_3: const #0s : i
        let s_19_3: i128 = 0;
        // D s_19_4: cmp-gt s_19_2 s_19_3
        let s_19_4: bool = ((s_19_2) > (s_19_3));
        // D s_19_5: not s_19_4
        let s_19_5: bool = !s_19_4;
        // N s_19_6: branch s_19_5 b25 b20
        if s_19_5 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1s : i
        let s_20_0: i128 = 1;
        // D s_20_1: read-var toaddress:u64
        let s_20_1: u64 = fn_state.toaddress;
        // D s_20_2: read-var setsize:u64
        let s_20_2: u64 = fn_state.setsize;
        // D s_20_3: call SETSizeChoice(s_20_1, s_20_2, s_20_0)
        let s_20_3: i128 = SETSizeChoice(state, tracer, s_20_1, s_20_2, s_20_0);
        // D s_20_4: write-var B <= s_20_3
        fn_state.B = s_20_3;
        // D s_20_5: read-var stagesetsize:u64
        let s_20_5: u64 = fn_state.stagesetsize;
        // D s_20_6: cast zx s_20_5 -> bv
        let s_20_6: Bits = Bits::new(s_20_5 as u128, 64u16);
        // D s_20_7: cast zx s_20_6 -> i
        let s_20_7: i128 = (s_20_6.value() as i128);
        // D s_20_8: read-var B:i
        let s_20_8: i128 = fn_state.B;
        // D s_20_9: cmp-le s_20_8 s_20_7
        let s_20_9: bool = ((s_20_8) <= (s_20_7));
        // N s_20_10: assert s_20_9
        let s_20_10: () = assert!(s_20_9);
        // D s_20_11: read-var B:i
        let s_20_11: i128 = fn_state.B;
        // D s_20_12: call __id(s_20_11)
        let s_20_12: i128 = u__id(state, tracer, s_20_11);
        // C s_20_13: const #8s : i
        let s_20_13: i128 = 8;
        // D s_20_14: mul s_20_12 s_20_13
        let s_20_14: i128 = ((s_20_12) * (s_20_13));
        // C s_20_15: const #0s : i
        let s_20_15: i128 = 0;
        // D s_20_16: cmp-ge s_20_14 s_20_15
        let s_20_16: bool = ((s_20_14) >= (s_20_15));
        // N s_20_17: assert s_20_16
        let s_20_17: () = assert!(s_20_16);
        // D s_20_18: read-var data:u8
        let s_20_18: u8 = fn_state.data;
        // D s_20_19: cast zx s_20_18 -> bv
        let s_20_19: Bits = Bits::new(s_20_18 as u128, 8u16);
        // D s_20_20: cast reint s_20_11 -> u64
        let s_20_20: u64 = (s_20_11 as u64);
        // D s_20_21: call replicate_bits_borealis_internal(s_20_19, s_20_20)
        let s_20_21: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_20_19,
            s_20_20,
        );
        // D s_20_22: read-var toaddress:u64
        let s_20_22: u64 = fn_state.toaddress;
        // D s_20_23: read-var accdesc:struct
        let s_20_23: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_20_24: call Mem_set(s_20_22, s_20_11, s_20_23, s_20_21)
        let s_20_24: () = Mem_set(state, tracer, s_20_22, s_20_11, s_20_23, s_20_21);
        // N s_20_25: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var toaddress:u64
        let s_21_0: u64 = fn_state.toaddress;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 64u16);
        // D s_21_2: read-var B:i
        let s_21_2: i128 = fn_state.B;
        // D s_21_3: cast cvt s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 128);
        // D s_21_4: add s_21_1 s_21_3
        let s_21_4: Bits = (s_21_1 + s_21_3);
        // D s_21_5: cast reint s_21_4 -> u64
        let s_21_5: u64 = (s_21_4.value() as u64);
        // D s_21_6: write-var toaddress <= s_21_5
        fn_state.toaddress = s_21_5;
        // D s_21_7: read-var setsize:u64
        let s_21_7: u64 = fn_state.setsize;
        // D s_21_8: cast zx s_21_7 -> bv
        let s_21_8: Bits = Bits::new(s_21_7 as u128, 64u16);
        // D s_21_9: read-var B:i
        let s_21_9: i128 = fn_state.B;
        // D s_21_10: cast cvt s_21_9 -> bv
        let s_21_10: Bits = Bits::new(s_21_9 as u128, 128);
        // D s_21_11: sub s_21_8 s_21_10
        let s_21_11: Bits = ((s_21_8) - (s_21_10));
        // D s_21_12: cast reint s_21_11 -> u64
        let s_21_12: u64 = (s_21_11.value() as u64);
        // D s_21_13: write-var setsize <= s_21_12
        fn_state.setsize = s_21_12;
        // D s_21_14: read-var stagesetsize:u64
        let s_21_14: u64 = fn_state.stagesetsize;
        // D s_21_15: cast zx s_21_14 -> bv
        let s_21_15: Bits = Bits::new(s_21_14 as u128, 64u16);
        // D s_21_16: read-var B:i
        let s_21_16: i128 = fn_state.B;
        // D s_21_17: cast cvt s_21_16 -> bv
        let s_21_17: Bits = Bits::new(s_21_16 as u128, 128);
        // D s_21_18: sub s_21_15 s_21_17
        let s_21_18: Bits = ((s_21_15) - (s_21_17));
        // D s_21_19: cast reint s_21_18 -> u64
        let s_21_19: u64 = (s_21_18.value() as u64);
        // D s_21_20: write-var stagesetsize <= s_21_19
        fn_state.stagesetsize = s_21_19;
        // D s_21_21: read-var stage:u32
        let s_21_21: u32 = fn_state.stage;
        // C s_21_22: const #0u : u32
        let s_21_22: u32 = 0;
        // D s_21_23: cmp-eq s_21_21 s_21_22
        let s_21_23: bool = ((s_21_21) == (s_21_22));
        // N s_21_24: branch s_21_23 b24 b22
        if s_21_23 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #64s : i64
        let s_24_0: i64 = 64;
        // D s_24_1: read-var n:i64
        let s_24_1: i64 = fn_state.n;
        // D s_24_2: cast zx s_24_1 -> i
        let s_24_2: i128 = (i128::try_from(s_24_1).unwrap());
        // D s_24_3: read-var setsize:u64
        let s_24_3: u64 = fn_state.setsize;
        // D s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 64u16);
        // D s_24_5: call X_set(s_24_2, s_24_0, s_24_4)
        let s_24_5: () = X_set(state, tracer, s_24_2, s_24_0, s_24_4);
        // C s_24_6: const #64s : i64
        let s_24_6: i64 = 64;
        // D s_24_7: read-var d:i64
        let s_24_7: i64 = fn_state.d;
        // D s_24_8: cast zx s_24_7 -> i
        let s_24_8: i128 = (i128::try_from(s_24_7).unwrap());
        // D s_24_9: read-var toaddress:u64
        let s_24_9: u64 = fn_state.toaddress;
        // D s_24_10: cast zx s_24_9 -> bv
        let s_24_10: Bits = Bits::new(s_24_9 as u128, 64u16);
        // D s_24_11: call X_set(s_24_8, s_24_6, s_24_10)
        let s_24_11: () = X_set(state, tracer, s_24_8, s_24_6, s_24_10);
        // N s_24_12: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var stage:u32
        let s_26_0: u32 = fn_state.stage;
        // C s_26_1: const #0u : u32
        let s_26_1: u32 = 0;
        // D s_26_2: cmp-eq s_26_0 s_26_1
        let s_26_2: bool = ((s_26_0) == (s_26_1));
        // N s_26_3: branch s_26_2 b28 b27
        if s_26_2 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #64s : i64
        let s_28_0: i64 = 64;
        // D s_28_1: read-var n:i64
        let s_28_1: i64 = fn_state.n;
        // D s_28_2: cast zx s_28_1 -> i
        let s_28_2: i128 = (i128::try_from(s_28_1).unwrap());
        // D s_28_3: read-var setsize:u64
        let s_28_3: u64 = fn_state.setsize;
        // D s_28_4: cast zx s_28_3 -> bv
        let s_28_4: Bits = Bits::new(s_28_3 as u128, 64u16);
        // D s_28_5: call X_set(s_28_2, s_28_0, s_28_4)
        let s_28_5: () = X_set(state, tracer, s_28_2, s_28_0, s_28_4);
        // C s_28_6: const #64s : i64
        let s_28_6: i64 = 64;
        // D s_28_7: read-var d:i64
        let s_28_7: i64 = fn_state.d;
        // D s_28_8: cast zx s_28_7 -> i
        let s_28_8: i128 = (i128::try_from(s_28_7).unwrap());
        // D s_28_9: read-var toaddress:u64
        let s_28_9: u64 = fn_state.toaddress;
        // D s_28_10: cast zx s_28_9 -> bv
        let s_28_10: Bits = Bits::new(s_28_9 as u128, 64u16);
        // D s_28_11: call X_set(s_28_8, s_28_6, s_28_10)
        let s_28_11: () = X_set(state, tracer, s_28_8, s_28_6, s_28_10);
        // C s_28_12: const #3s : i
        let s_28_12: i128 = 3;
        // D s_28_13: read-var nzcv:u8
        let s_28_13: u8 = fn_state.nzcv;
        // D s_28_14: cast zx s_28_13 -> bv
        let s_28_14: Bits = Bits::new(s_28_13 as u128, 4u16);
        // C s_28_15: const #1s : i64
        let s_28_15: i64 = 1;
        // C s_28_16: cast zx s_28_15 -> i
        let s_28_16: i128 = (i128::try_from(s_28_15).unwrap());
        // C s_28_17: const #0s : i
        let s_28_17: i128 = 0;
        // C s_28_18: add s_28_17 s_28_16
        let s_28_18: i128 = (s_28_17 + s_28_16);
        // D s_28_19: bit-extract s_28_14 s_28_12 s_28_18
        let s_28_19: Bits = (Bits::new(
            ((s_28_14) >> (s_28_12)).value(),
            u16::try_from(s_28_18).unwrap(),
        ));
        // D s_28_20: cast reint s_28_19 -> u8
        let s_28_20: bool = ((s_28_19.value()) != 0);
        // C s_28_21: const #16984u : u32
        let s_28_21: u32 = 16984;
        // N s_28_22: write-reg s_28_21 <= s_28_20
        let s_28_22: () = {
            state.write_register::<bool>(s_28_21 as isize, s_28_20);
            tracer.write_register(s_28_21 as isize, s_28_20);
        };
        // C s_28_23: const #2s : i
        let s_28_23: i128 = 2;
        // D s_28_24: read-var nzcv:u8
        let s_28_24: u8 = fn_state.nzcv;
        // D s_28_25: cast zx s_28_24 -> bv
        let s_28_25: Bits = Bits::new(s_28_24 as u128, 4u16);
        // C s_28_26: const #1s : i64
        let s_28_26: i64 = 1;
        // C s_28_27: cast zx s_28_26 -> i
        let s_28_27: i128 = (i128::try_from(s_28_26).unwrap());
        // C s_28_28: const #0s : i
        let s_28_28: i128 = 0;
        // C s_28_29: add s_28_28 s_28_27
        let s_28_29: i128 = (s_28_28 + s_28_27);
        // D s_28_30: bit-extract s_28_25 s_28_23 s_28_29
        let s_28_30: Bits = (Bits::new(
            ((s_28_25) >> (s_28_23)).value(),
            u16::try_from(s_28_29).unwrap(),
        ));
        // D s_28_31: cast reint s_28_30 -> u8
        let s_28_31: bool = ((s_28_30.value()) != 0);
        // C s_28_32: const #16997u : u32
        let s_28_32: u32 = 16997;
        // N s_28_33: write-reg s_28_32 <= s_28_31
        let s_28_33: () = {
            state.write_register::<bool>(s_28_32 as isize, s_28_31);
            tracer.write_register(s_28_32 as isize, s_28_31);
        };
        // C s_28_34: const #1s : i
        let s_28_34: i128 = 1;
        // D s_28_35: read-var nzcv:u8
        let s_28_35: u8 = fn_state.nzcv;
        // D s_28_36: cast zx s_28_35 -> bv
        let s_28_36: Bits = Bits::new(s_28_35 as u128, 4u16);
        // C s_28_37: const #1s : i64
        let s_28_37: i64 = 1;
        // C s_28_38: cast zx s_28_37 -> i
        let s_28_38: i128 = (i128::try_from(s_28_37).unwrap());
        // C s_28_39: const #0s : i
        let s_28_39: i128 = 0;
        // C s_28_40: add s_28_39 s_28_38
        let s_28_40: i128 = (s_28_39 + s_28_38);
        // D s_28_41: bit-extract s_28_36 s_28_34 s_28_40
        let s_28_41: Bits = (Bits::new(
            ((s_28_36) >> (s_28_34)).value(),
            u16::try_from(s_28_40).unwrap(),
        ));
        // D s_28_42: cast reint s_28_41 -> u8
        let s_28_42: bool = ((s_28_41.value()) != 0);
        // C s_28_43: const #16971u : u32
        let s_28_43: u32 = 16971;
        // N s_28_44: write-reg s_28_43 <= s_28_42
        let s_28_44: () = {
            state.write_register::<bool>(s_28_43 as isize, s_28_42);
            tracer.write_register(s_28_43 as isize, s_28_42);
        };
        // C s_28_45: const #0s : i
        let s_28_45: i128 = 0;
        // D s_28_46: read-var nzcv:u8
        let s_28_46: u8 = fn_state.nzcv;
        // D s_28_47: cast zx s_28_46 -> bv
        let s_28_47: Bits = Bits::new(s_28_46 as u128, 4u16);
        // C s_28_48: const #1s : i64
        let s_28_48: i64 = 1;
        // C s_28_49: cast zx s_28_48 -> i
        let s_28_49: i128 = (i128::try_from(s_28_48).unwrap());
        // C s_28_50: const #0s : i
        let s_28_50: i128 = 0;
        // C s_28_51: add s_28_50 s_28_49
        let s_28_51: i128 = (s_28_50 + s_28_49);
        // D s_28_52: bit-extract s_28_47 s_28_45 s_28_51
        let s_28_52: Bits = (Bits::new(
            ((s_28_47) >> (s_28_45)).value(),
            u16::try_from(s_28_51).unwrap(),
        ));
        // D s_28_53: cast reint s_28_52 -> u8
        let s_28_53: bool = ((s_28_52.value()) != 0);
        // C s_28_54: const #16996u : u32
        let s_28_54: u32 = 16996;
        // N s_28_55: write-reg s_28_54 <= s_28_53
        let s_28_55: () = {
            state.write_register::<bool>(s_28_54 as isize, s_28_53);
            tracer.write_register(s_28_54 as isize, s_28_53);
        };
        // N s_28_56: return
        return;
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
        // D s_30_0: read-var stagesetsize:u64
        let s_30_0: u64 = fn_state.stagesetsize;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 64u16);
        // D s_30_2: cast sx s_30_1 -> i
        let s_30_2: i128 = {
            let sign_bit = s_30_1.length() - 1;
            let mut result = s_30_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: const #0s : i
        let s_30_4: i128 = 0;
        // D s_30_5: cast zx s_30_3 -> i
        let s_30_5: i128 = (i128::try_from(s_30_3).unwrap());
        // D s_30_6: cmp-lt s_30_5 s_30_4
        let s_30_6: bool = ((s_30_5) < (s_30_4));
        // D s_30_7: not s_30_6
        let s_30_7: bool = !s_30_6;
        // N s_30_8: branch s_30_7 b36 b31
        if s_30_7 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1s : i
        let s_31_0: i128 = 1;
        // D s_31_1: read-var toaddress:u64
        let s_31_1: u64 = fn_state.toaddress;
        // D s_31_2: read-var setsize:u64
        let s_31_2: u64 = fn_state.setsize;
        // D s_31_3: call SETSizeChoice(s_31_1, s_31_2, s_31_0)
        let s_31_3: i128 = SETSizeChoice(state, tracer, s_31_1, s_31_2, s_31_0);
        // D s_31_4: write-var B <= s_31_3
        fn_state.B = s_31_3;
        // C s_31_5: const #1s : i
        let s_31_5: i128 = 1;
        // C s_31_6: neg s_31_5
        let s_31_6: i128 = -s_31_5;
        // C s_31_7: cast reint s_31_6 -> i64
        let s_31_7: i64 = (s_31_6 as i64);
        // D s_31_8: read-var stagesetsize:u64
        let s_31_8: u64 = fn_state.stagesetsize;
        // D s_31_9: cast zx s_31_8 -> bv
        let s_31_9: Bits = Bits::new(s_31_8 as u128, 64u16);
        // D s_31_10: cast sx s_31_9 -> i
        let s_31_10: i128 = {
            let sign_bit = s_31_9.length() - 1;
            let mut result = s_31_9.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_31_11: cast reint s_31_10 -> i64
        let s_31_11: i64 = (s_31_10 as i64);
        // C s_31_12: cast zx s_31_7 -> i
        let s_31_12: i128 = (i128::try_from(s_31_7).unwrap());
        // D s_31_13: cast zx s_31_11 -> i
        let s_31_13: i128 = (i128::try_from(s_31_11).unwrap());
        // D s_31_14: mul s_31_12 s_31_13
        let s_31_14: i128 = ((s_31_12) * (s_31_13));
        // D s_31_15: read-var B:i
        let s_31_15: i128 = fn_state.B;
        // D s_31_16: cmp-le s_31_15 s_31_14
        let s_31_16: bool = ((s_31_15) <= (s_31_14));
        // N s_31_17: assert s_31_16
        let s_31_17: () = assert!(s_31_16);
        // D s_31_18: read-var B:i
        let s_31_18: i128 = fn_state.B;
        // D s_31_19: call __id(s_31_18)
        let s_31_19: i128 = u__id(state, tracer, s_31_18);
        // C s_31_20: const #8s : i
        let s_31_20: i128 = 8;
        // D s_31_21: mul s_31_19 s_31_20
        let s_31_21: i128 = ((s_31_19) * (s_31_20));
        // C s_31_22: const #0s : i
        let s_31_22: i128 = 0;
        // D s_31_23: cmp-ge s_31_21 s_31_22
        let s_31_23: bool = ((s_31_21) >= (s_31_22));
        // N s_31_24: assert s_31_23
        let s_31_24: () = assert!(s_31_23);
        // D s_31_25: read-var toaddress:u64
        let s_31_25: u64 = fn_state.toaddress;
        // D s_31_26: cast zx s_31_25 -> bv
        let s_31_26: Bits = Bits::new(s_31_25 as u128, 64u16);
        // D s_31_27: read-var setsize:u64
        let s_31_27: u64 = fn_state.setsize;
        // D s_31_28: cast zx s_31_27 -> bv
        let s_31_28: Bits = Bits::new(s_31_27 as u128, 64u16);
        // D s_31_29: add s_31_26 s_31_28
        let s_31_29: Bits = (s_31_26 + s_31_28);
        // D s_31_30: cast reint s_31_29 -> u64
        let s_31_30: u64 = (s_31_29.value() as u64);
        // D s_31_31: read-var data:u8
        let s_31_31: u8 = fn_state.data;
        // D s_31_32: cast zx s_31_31 -> bv
        let s_31_32: Bits = Bits::new(s_31_31 as u128, 8u16);
        // D s_31_33: cast reint s_31_18 -> u64
        let s_31_33: u64 = (s_31_18 as u64);
        // D s_31_34: call replicate_bits_borealis_internal(s_31_32, s_31_33)
        let s_31_34: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_31_32,
            s_31_33,
        );
        // D s_31_35: read-var accdesc:struct
        let s_31_35: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_31_36: call Mem_set(s_31_30, s_31_18, s_31_35, s_31_34)
        let s_31_36: () = Mem_set(state, tracer, s_31_30, s_31_18, s_31_35, s_31_34);
        // N s_31_37: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var setsize:u64
        let s_32_0: u64 = fn_state.setsize;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 64u16);
        // D s_32_2: read-var B:i
        let s_32_2: i128 = fn_state.B;
        // D s_32_3: cast cvt s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 128);
        // D s_32_4: add s_32_1 s_32_3
        let s_32_4: Bits = (s_32_1 + s_32_3);
        // D s_32_5: cast reint s_32_4 -> u64
        let s_32_5: u64 = (s_32_4.value() as u64);
        // D s_32_6: write-var setsize <= s_32_5
        fn_state.setsize = s_32_5;
        // D s_32_7: read-var stagesetsize:u64
        let s_32_7: u64 = fn_state.stagesetsize;
        // D s_32_8: cast zx s_32_7 -> bv
        let s_32_8: Bits = Bits::new(s_32_7 as u128, 64u16);
        // D s_32_9: read-var B:i
        let s_32_9: i128 = fn_state.B;
        // D s_32_10: cast cvt s_32_9 -> bv
        let s_32_10: Bits = Bits::new(s_32_9 as u128, 128);
        // D s_32_11: add s_32_8 s_32_10
        let s_32_11: Bits = (s_32_8 + s_32_10);
        // D s_32_12: cast reint s_32_11 -> u64
        let s_32_12: u64 = (s_32_11.value() as u64);
        // D s_32_13: write-var stagesetsize <= s_32_12
        fn_state.stagesetsize = s_32_12;
        // D s_32_14: read-var stage:u32
        let s_32_14: u32 = fn_state.stage;
        // C s_32_15: const #0u : u32
        let s_32_15: u32 = 0;
        // D s_32_16: cmp-eq s_32_14 s_32_15
        let s_32_16: bool = ((s_32_14) == (s_32_15));
        // N s_32_17: branch s_32_16 b35 b33
        if s_32_16 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #64s : i64
        let s_35_0: i64 = 64;
        // D s_35_1: read-var n:i64
        let s_35_1: i64 = fn_state.n;
        // D s_35_2: cast zx s_35_1 -> i
        let s_35_2: i128 = (i128::try_from(s_35_1).unwrap());
        // D s_35_3: read-var setsize:u64
        let s_35_3: u64 = fn_state.setsize;
        // D s_35_4: cast zx s_35_3 -> bv
        let s_35_4: Bits = Bits::new(s_35_3 as u128, 64u16);
        // D s_35_5: call X_set(s_35_2, s_35_0, s_35_4)
        let s_35_5: () = X_set(state, tracer, s_35_2, s_35_0, s_35_4);
        // N s_35_6: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call SPESampleMemSet(s_37_0)
        let s_37_1: () = SPESampleMemSet(state, tracer, s_37_0);
        // N s_37_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // C s_38_1: const #1u : u8
        let s_38_1: bool = true;
        // D s_38_2: read-var d:i64
        let s_38_2: i64 = fn_state.d;
        // D s_38_3: cast zx s_38_2 -> i
        let s_38_3: i128 = (i128::try_from(s_38_2).unwrap());
        // D s_38_4: read-var s:i64
        let s_38_4: i64 = fn_state.s;
        // D s_38_5: cast zx s_38_4 -> i
        let s_38_5: i128 = (i128::try_from(s_38_4).unwrap());
        // D s_38_6: read-var n:i64
        let s_38_6: i64 = fn_state.n;
        // D s_38_7: cast zx s_38_6 -> i
        let s_38_7: i128 = (i128::try_from(s_38_6).unwrap());
        // D s_38_8: read-var supports_option_a:u8
        let s_38_8: bool = fn_state.supports_option_a;
        // D s_38_9: read-var options_name:u8
        let s_38_9: u8 = fn_state.options_name;
        // C s_38_10: const #0u : u8
        let s_38_10: bool = false;
        // D s_38_11: call MismatchedMemSetException(s_38_8, s_38_3, s_38_5, s_38_7, s_38_0, s_38_1, s_38_9, s_38_10)
        let s_38_11: () = MismatchedMemSetException(
            state,
            tracer,
            s_38_8,
            s_38_3,
            s_38_5,
            s_38_7,
            s_38_0,
            s_38_1,
            s_38_9,
            s_38_10,
        );
        // N s_38_12: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#167876 <= s_39_0
        fn_state.gs_167876 = s_39_0;
        // N s_39_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var setsize:u64
        let s_40_0: u64 = fn_state.setsize;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 64u16);
        // D s_40_2: read-var postsize:u64
        let s_40_2: u64 = fn_state.postsize;
        // D s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 64u16);
        // D s_40_4: sub s_40_1 s_40_3
        let s_40_4: Bits = ((s_40_1) - (s_40_3));
        // D s_40_5: cast reint s_40_4 -> u64
        let s_40_5: u64 = (s_40_4.value() as u64);
        // D s_40_6: write-var stagesetsize <= s_40_5
        fn_state.stagesetsize = s_40_5;
        // D s_40_7: read-var toaddress:u64
        let s_40_7: u64 = fn_state.toaddress;
        // D s_40_8: read-var setsize:u64
        let s_40_8: u64 = fn_state.setsize;
        // C s_40_9: const #0u : u8
        let s_40_9: bool = false;
        // D s_40_10: call MemSetParametersIllformedM(s_40_7, s_40_8, s_40_9)
        let s_40_10: bool = MemSetParametersIllformedM(
            state,
            tracer,
            s_40_7,
            s_40_8,
            s_40_9,
        );
        // N s_40_11: branch s_40_10 b43 b41
        if s_40_10 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_41_0: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_42_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // C s_43_1: const #0u : u8
        let s_43_1: bool = false;
        // D s_43_2: read-var d:i64
        let s_43_2: i64 = fn_state.d;
        // D s_43_3: cast zx s_43_2 -> i
        let s_43_3: i128 = (i128::try_from(s_43_2).unwrap());
        // D s_43_4: read-var s:i64
        let s_43_4: i64 = fn_state.s;
        // D s_43_5: cast zx s_43_4 -> i
        let s_43_5: i128 = (i128::try_from(s_43_4).unwrap());
        // D s_43_6: read-var n:i64
        let s_43_6: i64 = fn_state.n;
        // D s_43_7: cast zx s_43_6 -> i
        let s_43_7: i128 = (i128::try_from(s_43_6).unwrap());
        // D s_43_8: read-var supports_option_a:u8
        let s_43_8: bool = fn_state.supports_option_a;
        // D s_43_9: read-var options_name:u8
        let s_43_9: u8 = fn_state.options_name;
        // C s_43_10: const #0u : u8
        let s_43_10: bool = false;
        // D s_43_11: call MismatchedMemSetException(s_43_8, s_43_3, s_43_5, s_43_7, s_43_0, s_43_1, s_43_9, s_43_10)
        let s_43_11: () = MismatchedMemSetException(
            state,
            tracer,
            s_43_8,
            s_43_3,
            s_43_5,
            s_43_7,
            s_43_0,
            s_43_1,
            s_43_9,
            s_43_10,
        );
        // N s_43_12: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var supports_option_a:u8
        let s_44_0: bool = fn_state.supports_option_a;
        // N s_44_1: branch s_44_0 b49 b45
        if s_44_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #1s : i
        let s_45_0: i128 = 1;
        // D s_45_1: read-var nzcv:u8
        let s_45_1: u8 = fn_state.nzcv;
        // D s_45_2: cast zx s_45_1 -> bv
        let s_45_2: Bits = Bits::new(s_45_1 as u128, 4u16);
        // C s_45_3: const #1u : u64
        let s_45_3: u64 = 1;
        // D s_45_4: bit-extract s_45_2 s_45_0 s_45_3
        let s_45_4: Bits = (Bits::new(
            ((s_45_2) >> (s_45_0)).value(),
            u16::try_from(s_45_3).unwrap(),
        ));
        // D s_45_5: cast reint s_45_4 -> u8
        let s_45_5: bool = ((s_45_4.value()) != 0);
        // C s_45_6: const #0s : i
        let s_45_6: i128 = 0;
        // C s_45_7: const #0u : u64
        let s_45_7: u64 = 0;
        // D s_45_8: cast zx s_45_5 -> u64
        let s_45_8: u64 = (s_45_5 as u64);
        // C s_45_9: const #1u : u64
        let s_45_9: u64 = 1;
        // D s_45_10: and s_45_8 s_45_9
        let s_45_10: u64 = ((s_45_8) & (s_45_9));
        // D s_45_11: cmp-eq s_45_10 s_45_9
        let s_45_11: bool = ((s_45_10) == (s_45_9));
        // D s_45_12: lsl s_45_8 s_45_6
        let s_45_12: u64 = s_45_8 << s_45_6;
        // D s_45_13: or s_45_7 s_45_12
        let s_45_13: u64 = ((s_45_7) | (s_45_12));
        // D s_45_14: cmpl s_45_12
        let s_45_14: u64 = !s_45_12;
        // D s_45_15: and s_45_7 s_45_14
        let s_45_15: u64 = ((s_45_7) & (s_45_14));
        // D s_45_16: select s_45_11 s_45_13 s_45_15
        let s_45_16: u64 = if s_45_11 { s_45_13 } else { s_45_15 };
        // D s_45_17: cast trunc s_45_16 -> u8
        let s_45_17: bool = ((s_45_16) != 0);
        // D s_45_18: cast zx s_45_17 -> bv
        let s_45_18: Bits = Bits::new(s_45_17 as u128, 1u16);
        // C s_45_19: const #0u : u8
        let s_45_19: bool = false;
        // C s_45_20: cast zx s_45_19 -> bv
        let s_45_20: Bits = Bits::new(s_45_19 as u128, 1u16);
        // D s_45_21: cmp-eq s_45_18 s_45_20
        let s_45_21: bool = ((s_45_18) == (s_45_20));
        // N s_45_22: branch s_45_21 b48 b46
        if s_45_21 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_46_0: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_47_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #1u : u8
        let s_48_0: bool = true;
        // D s_48_1: read-var stage:u32
        let s_48_1: u32 = fn_state.stage;
        // C s_48_2: const #2u : u32
        let s_48_2: u32 = 2;
        // D s_48_3: cmp-eq s_48_1 s_48_2
        let s_48_3: bool = ((s_48_1) == (s_48_2));
        // D s_48_4: read-var d:i64
        let s_48_4: i64 = fn_state.d;
        // D s_48_5: cast zx s_48_4 -> i
        let s_48_5: i128 = (i128::try_from(s_48_4).unwrap());
        // D s_48_6: read-var s:i64
        let s_48_6: i64 = fn_state.s;
        // D s_48_7: cast zx s_48_6 -> i
        let s_48_7: i128 = (i128::try_from(s_48_6).unwrap());
        // D s_48_8: read-var n:i64
        let s_48_8: i64 = fn_state.n;
        // D s_48_9: cast zx s_48_8 -> i
        let s_48_9: i128 = (i128::try_from(s_48_8).unwrap());
        // D s_48_10: read-var supports_option_a:u8
        let s_48_10: bool = fn_state.supports_option_a;
        // D s_48_11: read-var options_name:u8
        let s_48_11: u8 = fn_state.options_name;
        // C s_48_12: const #0u : u8
        let s_48_12: bool = false;
        // D s_48_13: call MismatchedMemSetException(s_48_10, s_48_5, s_48_7, s_48_9, s_48_0, s_48_3, s_48_11, s_48_12)
        let s_48_13: () = MismatchedMemSetException(
            state,
            tracer,
            s_48_10,
            s_48_5,
            s_48_7,
            s_48_9,
            s_48_0,
            s_48_3,
            s_48_11,
            s_48_12,
        );
        // N s_48_14: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1s : i
        let s_49_0: i128 = 1;
        // D s_49_1: read-var nzcv:u8
        let s_49_1: u8 = fn_state.nzcv;
        // D s_49_2: cast zx s_49_1 -> bv
        let s_49_2: Bits = Bits::new(s_49_1 as u128, 4u16);
        // C s_49_3: const #1u : u64
        let s_49_3: u64 = 1;
        // D s_49_4: bit-extract s_49_2 s_49_0 s_49_3
        let s_49_4: Bits = (Bits::new(
            ((s_49_2) >> (s_49_0)).value(),
            u16::try_from(s_49_3).unwrap(),
        ));
        // D s_49_5: cast reint s_49_4 -> u8
        let s_49_5: bool = ((s_49_4.value()) != 0);
        // C s_49_6: const #0s : i
        let s_49_6: i128 = 0;
        // C s_49_7: const #0u : u64
        let s_49_7: u64 = 0;
        // D s_49_8: cast zx s_49_5 -> u64
        let s_49_8: u64 = (s_49_5 as u64);
        // C s_49_9: const #1u : u64
        let s_49_9: u64 = 1;
        // D s_49_10: and s_49_8 s_49_9
        let s_49_10: u64 = ((s_49_8) & (s_49_9));
        // D s_49_11: cmp-eq s_49_10 s_49_9
        let s_49_11: bool = ((s_49_10) == (s_49_9));
        // D s_49_12: lsl s_49_8 s_49_6
        let s_49_12: u64 = s_49_8 << s_49_6;
        // D s_49_13: or s_49_7 s_49_12
        let s_49_13: u64 = ((s_49_7) | (s_49_12));
        // D s_49_14: cmpl s_49_12
        let s_49_14: u64 = !s_49_12;
        // D s_49_15: and s_49_7 s_49_14
        let s_49_15: u64 = ((s_49_7) & (s_49_14));
        // D s_49_16: select s_49_11 s_49_13 s_49_15
        let s_49_16: u64 = if s_49_11 { s_49_13 } else { s_49_15 };
        // D s_49_17: cast trunc s_49_16 -> u8
        let s_49_17: bool = ((s_49_16) != 0);
        // D s_49_18: cast zx s_49_17 -> bv
        let s_49_18: Bits = Bits::new(s_49_17 as u128, 1u16);
        // C s_49_19: const #1u : u8
        let s_49_19: bool = true;
        // C s_49_20: cast zx s_49_19 -> bv
        let s_49_20: Bits = Bits::new(s_49_19 as u128, 1u16);
        // D s_49_21: cmp-eq s_49_18 s_49_20
        let s_49_21: bool = ((s_49_18) == (s_49_20));
        // N s_49_22: branch s_49_21 b52 b50
        if s_49_21 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_50_0: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_51_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #1u : u8
        let s_52_0: bool = true;
        // D s_52_1: read-var stage:u32
        let s_52_1: u32 = fn_state.stage;
        // C s_52_2: const #2u : u32
        let s_52_2: u32 = 2;
        // D s_52_3: cmp-eq s_52_1 s_52_2
        let s_52_3: bool = ((s_52_1) == (s_52_2));
        // D s_52_4: read-var d:i64
        let s_52_4: i64 = fn_state.d;
        // D s_52_5: cast zx s_52_4 -> i
        let s_52_5: i128 = (i128::try_from(s_52_4).unwrap());
        // D s_52_6: read-var s:i64
        let s_52_6: i64 = fn_state.s;
        // D s_52_7: cast zx s_52_6 -> i
        let s_52_7: i128 = (i128::try_from(s_52_6).unwrap());
        // D s_52_8: read-var n:i64
        let s_52_8: i64 = fn_state.n;
        // D s_52_9: cast zx s_52_8 -> i
        let s_52_9: i128 = (i128::try_from(s_52_8).unwrap());
        // D s_52_10: read-var supports_option_a:u8
        let s_52_10: bool = fn_state.supports_option_a;
        // D s_52_11: read-var options_name:u8
        let s_52_11: u8 = fn_state.options_name;
        // C s_52_12: const #0u : u8
        let s_52_12: bool = false;
        // D s_52_13: call MismatchedMemSetException(s_52_10, s_52_5, s_52_7, s_52_9, s_52_0, s_52_3, s_52_11, s_52_12)
        let s_52_13: () = MismatchedMemSetException(
            state,
            tracer,
            s_52_10,
            s_52_5,
            s_52_7,
            s_52_9,
            s_52_0,
            s_52_3,
            s_52_11,
            s_52_12,
        );
        // N s_52_14: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // D s_53_1: write-var gs#167868 <= s_53_0
        fn_state.gs_167868 = s_53_0;
        // N s_53_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // D s_54_1: write-var gs#167865 <= s_54_0
        fn_state.gs_167865 = s_54_0;
        // N s_54_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #63s : i
        let s_55_0: i128 = 63;
        // D s_55_1: read-var setsize:u64
        let s_55_1: u64 = fn_state.setsize;
        // D s_55_2: cast zx s_55_1 -> bv
        let s_55_2: Bits = Bits::new(s_55_1 as u128, 64u16);
        // C s_55_3: const #1u : u64
        let s_55_3: u64 = 1;
        // D s_55_4: bit-extract s_55_2 s_55_0 s_55_3
        let s_55_4: Bits = (Bits::new(
            ((s_55_2) >> (s_55_0)).value(),
            u16::try_from(s_55_3).unwrap(),
        ));
        // D s_55_5: cast reint s_55_4 -> u8
        let s_55_5: bool = ((s_55_4.value()) != 0);
        // C s_55_6: const #0s : i
        let s_55_6: i128 = 0;
        // C s_55_7: const #0u : u64
        let s_55_7: u64 = 0;
        // D s_55_8: cast zx s_55_5 -> u64
        let s_55_8: u64 = (s_55_5 as u64);
        // C s_55_9: const #1u : u64
        let s_55_9: u64 = 1;
        // D s_55_10: and s_55_8 s_55_9
        let s_55_10: u64 = ((s_55_8) & (s_55_9));
        // D s_55_11: cmp-eq s_55_10 s_55_9
        let s_55_11: bool = ((s_55_10) == (s_55_9));
        // D s_55_12: lsl s_55_8 s_55_6
        let s_55_12: u64 = s_55_8 << s_55_6;
        // D s_55_13: or s_55_7 s_55_12
        let s_55_13: u64 = ((s_55_7) | (s_55_12));
        // D s_55_14: cmpl s_55_12
        let s_55_14: u64 = !s_55_12;
        // D s_55_15: and s_55_7 s_55_14
        let s_55_15: u64 = ((s_55_7) & (s_55_14));
        // D s_55_16: select s_55_11 s_55_13 s_55_15
        let s_55_16: u64 = if s_55_11 { s_55_13 } else { s_55_15 };
        // D s_55_17: cast trunc s_55_16 -> u8
        let s_55_17: bool = ((s_55_16) != 0);
        // D s_55_18: cast zx s_55_17 -> bv
        let s_55_18: Bits = Bits::new(s_55_17 as u128, 1u16);
        // C s_55_19: const #1u : u8
        let s_55_19: bool = true;
        // C s_55_20: cast zx s_55_19 -> bv
        let s_55_20: Bits = Bits::new(s_55_19 as u128, 1u16);
        // D s_55_21: cmp-eq s_55_18 s_55_20
        let s_55_21: bool = ((s_55_18) == (s_55_20));
        // N s_55_22: branch s_55_21 b66 b56
        if s_55_21 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_56_0: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var supports_option_a:u8
        let s_57_0: bool = fn_state.supports_option_a;
        // N s_57_1: branch s_57_0 b65 b58
        if s_57_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #2u : u8
        let s_58_0: u8 = 2;
        // D s_58_1: write-var nzcv <= s_58_0
        fn_state.nzcv = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var toaddress:u64
        let s_59_0: u64 = fn_state.toaddress;
        // D s_59_1: read-var setsize:u64
        let s_59_1: u64 = fn_state.setsize;
        // C s_59_2: const #0u : u8
        let s_59_2: bool = false;
        // D s_59_3: call SETPreSizeChoice(s_59_0, s_59_1, s_59_2)
        let s_59_3: u64 = SETPreSizeChoice(state, tracer, s_59_0, s_59_1, s_59_2);
        // D s_59_4: write-var stagesetsize <= s_59_3
        fn_state.stagesetsize = s_59_3;
        // C s_59_5: const #63s : i
        let s_59_5: i128 = 63;
        // D s_59_6: read-var stagesetsize:u64
        let s_59_6: u64 = fn_state.stagesetsize;
        // D s_59_7: cast zx s_59_6 -> bv
        let s_59_7: Bits = Bits::new(s_59_6 as u128, 64u16);
        // C s_59_8: const #1u : u64
        let s_59_8: u64 = 1;
        // D s_59_9: bit-extract s_59_7 s_59_5 s_59_8
        let s_59_9: Bits = (Bits::new(
            ((s_59_7) >> (s_59_5)).value(),
            u16::try_from(s_59_8).unwrap(),
        ));
        // D s_59_10: cast reint s_59_9 -> u8
        let s_59_10: bool = ((s_59_9.value()) != 0);
        // C s_59_11: const #0s : i
        let s_59_11: i128 = 0;
        // C s_59_12: const #0u : u64
        let s_59_12: u64 = 0;
        // D s_59_13: cast zx s_59_10 -> u64
        let s_59_13: u64 = (s_59_10 as u64);
        // C s_59_14: const #1u : u64
        let s_59_14: u64 = 1;
        // D s_59_15: and s_59_13 s_59_14
        let s_59_15: u64 = ((s_59_13) & (s_59_14));
        // D s_59_16: cmp-eq s_59_15 s_59_14
        let s_59_16: bool = ((s_59_15) == (s_59_14));
        // D s_59_17: lsl s_59_13 s_59_11
        let s_59_17: u64 = s_59_13 << s_59_11;
        // D s_59_18: or s_59_12 s_59_17
        let s_59_18: u64 = ((s_59_12) | (s_59_17));
        // D s_59_19: cmpl s_59_17
        let s_59_19: u64 = !s_59_17;
        // D s_59_20: and s_59_12 s_59_19
        let s_59_20: u64 = ((s_59_12) & (s_59_19));
        // D s_59_21: select s_59_16 s_59_18 s_59_20
        let s_59_21: u64 = if s_59_16 { s_59_18 } else { s_59_20 };
        // D s_59_22: cast trunc s_59_21 -> u8
        let s_59_22: bool = ((s_59_21) != 0);
        // C s_59_23: const #63s : i
        let s_59_23: i128 = 63;
        // D s_59_24: read-var setsize:u64
        let s_59_24: u64 = fn_state.setsize;
        // D s_59_25: cast zx s_59_24 -> bv
        let s_59_25: Bits = Bits::new(s_59_24 as u128, 64u16);
        // C s_59_26: const #1u : u64
        let s_59_26: u64 = 1;
        // D s_59_27: bit-extract s_59_25 s_59_23 s_59_26
        let s_59_27: Bits = (Bits::new(
            ((s_59_25) >> (s_59_23)).value(),
            u16::try_from(s_59_26).unwrap(),
        ));
        // D s_59_28: cast reint s_59_27 -> u8
        let s_59_28: bool = ((s_59_27.value()) != 0);
        // C s_59_29: const #0s : i
        let s_59_29: i128 = 0;
        // C s_59_30: const #0u : u64
        let s_59_30: u64 = 0;
        // D s_59_31: cast zx s_59_28 -> u64
        let s_59_31: u64 = (s_59_28 as u64);
        // C s_59_32: const #1u : u64
        let s_59_32: u64 = 1;
        // D s_59_33: and s_59_31 s_59_32
        let s_59_33: u64 = ((s_59_31) & (s_59_32));
        // D s_59_34: cmp-eq s_59_33 s_59_32
        let s_59_34: bool = ((s_59_33) == (s_59_32));
        // D s_59_35: lsl s_59_31 s_59_29
        let s_59_35: u64 = s_59_31 << s_59_29;
        // D s_59_36: or s_59_30 s_59_35
        let s_59_36: u64 = ((s_59_30) | (s_59_35));
        // D s_59_37: cmpl s_59_35
        let s_59_37: u64 = !s_59_35;
        // D s_59_38: and s_59_30 s_59_37
        let s_59_38: u64 = ((s_59_30) & (s_59_37));
        // D s_59_39: select s_59_34 s_59_36 s_59_38
        let s_59_39: u64 = if s_59_34 { s_59_36 } else { s_59_38 };
        // D s_59_40: cast trunc s_59_39 -> u8
        let s_59_40: bool = ((s_59_39) != 0);
        // D s_59_41: cast zx s_59_22 -> bv
        let s_59_41: Bits = Bits::new(s_59_22 as u128, 1u16);
        // D s_59_42: cast zx s_59_40 -> bv
        let s_59_42: Bits = Bits::new(s_59_40 as u128, 1u16);
        // D s_59_43: cmp-eq s_59_41 s_59_42
        let s_59_43: bool = ((s_59_41) == (s_59_42));
        // N s_59_44: branch s_59_43 b64 b60
        if s_59_43 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #64s : i
        let s_60_0: i128 = 64;
        // S s_60_1: call Zeros(s_60_0)
        let s_60_1: Bits = Zeros(state, tracer, s_60_0);
        // S s_60_2: cast reint s_60_1 -> u64
        let s_60_2: u64 = (s_60_1.value() as u64);
        // D s_60_3: read-var stagesetsize:u64
        let s_60_3: u64 = fn_state.stagesetsize;
        // D s_60_4: cast zx s_60_3 -> bv
        let s_60_4: Bits = Bits::new(s_60_3 as u128, 64u16);
        // S s_60_5: cast zx s_60_2 -> bv
        let s_60_5: Bits = Bits::new(s_60_2 as u128, 64u16);
        // D s_60_6: cmp-eq s_60_4 s_60_5
        let s_60_6: bool = ((s_60_4) == (s_60_5));
        // D s_60_7: write-var gs#167888 <= s_60_6
        fn_state.gs_167888 = s_60_6;
        // N s_60_8: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#167888:u8
        let s_61_0: bool = fn_state.gs_167888;
        // N s_61_1: assert s_61_0
        let s_61_1: () = assert!(s_61_0);
        // D s_61_2: read-var setsize:u64
        let s_61_2: u64 = fn_state.setsize;
        // D s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 64u16);
        // D s_61_4: cast sx s_61_3 -> i
        let s_61_4: i128 = {
            let sign_bit = s_61_3.length() - 1;
            let mut result = s_61_3.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_61_5: cast reint s_61_4 -> i64
        let s_61_5: i64 = (s_61_4 as i64);
        // C s_61_6: const #0s : i
        let s_61_6: i128 = 0;
        // D s_61_7: cast zx s_61_5 -> i
        let s_61_7: i128 = (i128::try_from(s_61_5).unwrap());
        // D s_61_8: cmp-gt s_61_7 s_61_6
        let s_61_8: bool = ((s_61_7) > (s_61_6));
        // N s_61_9: branch s_61_8 b63 b62
        if s_61_8 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var stagesetsize:u64
        let s_62_0: u64 = fn_state.stagesetsize;
        // D s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 64u16);
        // D s_62_2: cast sx s_62_1 -> i
        let s_62_2: i128 = {
            let sign_bit = s_62_1.length() - 1;
            let mut result = s_62_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_62_3: cast reint s_62_2 -> i64
        let s_62_3: i64 = (s_62_2 as i64);
        // D s_62_4: read-var setsize:u64
        let s_62_4: u64 = fn_state.setsize;
        // D s_62_5: cast zx s_62_4 -> bv
        let s_62_5: Bits = Bits::new(s_62_4 as u128, 64u16);
        // D s_62_6: cast sx s_62_5 -> i
        let s_62_6: i128 = {
            let sign_bit = s_62_5.length() - 1;
            let mut result = s_62_5.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_62_7: cast reint s_62_6 -> i64
        let s_62_7: i64 = (s_62_6 as i64);
        // D s_62_8: cast zx s_62_3 -> i
        let s_62_8: i128 = (i128::try_from(s_62_3).unwrap());
        // D s_62_9: cast zx s_62_7 -> i
        let s_62_9: i128 = (i128::try_from(s_62_7).unwrap());
        // D s_62_10: cmp-ge s_62_8 s_62_9
        let s_62_10: bool = ((s_62_8) >= (s_62_9));
        // N s_62_11: assert s_62_10
        let s_62_11: () = assert!(s_62_10);
        // N s_62_12: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var stagesetsize:u64
        let s_63_0: u64 = fn_state.stagesetsize;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 64u16);
        // D s_63_2: cast sx s_63_1 -> i
        let s_63_2: i128 = {
            let sign_bit = s_63_1.length() - 1;
            let mut result = s_63_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_63_3: cast reint s_63_2 -> i64
        let s_63_3: i64 = (s_63_2 as i64);
        // D s_63_4: read-var setsize:u64
        let s_63_4: u64 = fn_state.setsize;
        // D s_63_5: cast zx s_63_4 -> bv
        let s_63_5: Bits = Bits::new(s_63_4 as u128, 64u16);
        // D s_63_6: cast sx s_63_5 -> i
        let s_63_6: i128 = {
            let sign_bit = s_63_5.length() - 1;
            let mut result = s_63_5.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_63_7: cast reint s_63_6 -> i64
        let s_63_7: i64 = (s_63_6 as i64);
        // D s_63_8: cast zx s_63_3 -> i
        let s_63_8: i128 = (i128::try_from(s_63_3).unwrap());
        // D s_63_9: cast zx s_63_7 -> i
        let s_63_9: i128 = (i128::try_from(s_63_7).unwrap());
        // D s_63_10: cmp-le s_63_8 s_63_9
        let s_63_10: bool = ((s_63_8) <= (s_63_9));
        // N s_63_11: assert s_63_10
        let s_63_11: () = assert!(s_63_10);
        // N s_63_12: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #1u : u8
        let s_64_0: bool = true;
        // D s_64_1: write-var gs#167888 <= s_64_0
        fn_state.gs_167888 = s_64_0;
        // N s_64_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #0u : u8
        let s_65_0: u8 = 0;
        // D s_65_1: write-var nzcv <= s_65_0
        fn_state.nzcv = s_65_0;
        // D s_65_2: read-var toaddress:u64
        let s_65_2: u64 = fn_state.toaddress;
        // D s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 64u16);
        // D s_65_4: read-var setsize:u64
        let s_65_4: u64 = fn_state.setsize;
        // D s_65_5: cast zx s_65_4 -> bv
        let s_65_5: Bits = Bits::new(s_65_4 as u128, 64u16);
        // D s_65_6: add s_65_3 s_65_5
        let s_65_6: Bits = (s_65_3 + s_65_5);
        // D s_65_7: cast reint s_65_6 -> u64
        let s_65_7: u64 = (s_65_6.value() as u64);
        // D s_65_8: write-var toaddress <= s_65_7
        fn_state.toaddress = s_65_7;
        // C s_65_9: const #64s : i
        let s_65_9: i128 = 64;
        // S s_65_10: call Zeros(s_65_9)
        let s_65_10: Bits = Zeros(state, tracer, s_65_9);
        // S s_65_11: cast reint s_65_10 -> u64
        let s_65_11: u64 = (s_65_10.value() as u64);
        // S s_65_12: cast zx s_65_11 -> bv
        let s_65_12: Bits = Bits::new(s_65_11 as u128, 64u16);
        // D s_65_13: read-var setsize:u64
        let s_65_13: u64 = fn_state.setsize;
        // D s_65_14: cast zx s_65_13 -> bv
        let s_65_14: Bits = Bits::new(s_65_13 as u128, 64u16);
        // D s_65_15: sub s_65_12 s_65_14
        let s_65_15: Bits = ((s_65_12) - (s_65_14));
        // D s_65_16: cast reint s_65_15 -> u64
        let s_65_16: u64 = (s_65_15.value() as u64);
        // D s_65_17: write-var setsize <= s_65_16
        fn_state.setsize = s_65_16;
        // N s_65_18: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #9223372036854775807u : u64
        let s_66_0: u64 = 9223372036854775807;
        // D s_66_1: write-var setsize <= s_66_0
        fn_state.setsize = s_66_0;
        // N s_66_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #() : ()
        let s_67_0: () = ();
        // S s_67_1: call AArch64_IsUnprivAccessPriv(s_67_0)
        let s_67_1: bool = AArch64_IsUnprivAccessPriv(state, tracer, s_67_0);
        // D s_67_2: write-var privileged <= s_67_1
        fn_state.privileged = s_67_1;
        // N s_67_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
