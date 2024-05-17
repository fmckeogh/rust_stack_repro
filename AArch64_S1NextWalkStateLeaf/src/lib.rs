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
use AArch64_S1ApplyOutputPerms::*;
use AArch64_MAIRAttr::*;
use AArch64_LeafBase::*;
use DecodePASpace::*;
use HasUnprivileged::*;
use HaveSecureState::*;
use S1DecodeMemAttrs::*;
use AArch64_ContiguousBit::*;
use common::*;
pub fn AArch64_S1NextWalkStateLeaf<T: Tracer>(
    state: &mut State,
    tracer: &T,
    currentstate: ProductType96e7acababe246a1,
    s2fs1mro: bool,
    regime: u32,
    ss: u32,
    walkparams: ProductTypeef284266e139aee2,
    descriptor: Bits,
) -> ProductType96e7acababe246a1 {
    #[derive(Default)]
    struct FunctionState {
        gs_18044: bool,
        nextstate: ProductType96e7acababe246a1,
        gs_17974: bool,
        u_675: bool,
        ga_13491: ProductTypeda0231e9dc169f81,
        attrindx: u8,
        baseaddress: ProductTypeda0231e9dc169f81,
        gs_18003: bool,
        gs_18055: bool,
        sh: u8,
        ga_13456: ProductTypeda0231e9dc169f81,
        ga_13505: ProductTypeda0231e9dc169f81,
        split_vec: u8,
        gs_17975: bool,
        protectedbit: bool,
        ga_13555: ProductTypeda0231e9dc169f81,
        ns: bool,
        gs_18043: bool,
        gs_18042: bool,
        ga_13468: ProductTypeda0231e9dc169f81,
        currentstate: ProductType96e7acababe246a1,
        s2fs1mro: bool,
        regime: u32,
        ss: u32,
        walkparams: ProductTypeef284266e139aee2,
        descriptor: Bits,
    }
    let fn_state = FunctionState {
        currentstate,
        s2fs1mro,
        regime,
        ss,
        walkparams,
        descriptor,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_0_0: read-var walkparams.3:struct
        let s_0_0: bool = fn_state.walkparams._3;
        // D s_0_1: read-var walkparams.7:struct
        let s_0_1: bool = fn_state.walkparams._7;
        // D s_0_2: read-var walkparams.36:struct
        let s_0_2: u32 = fn_state.walkparams._36;
        // D s_0_3: read-var currentstate.6:struct
        let s_0_3: i128 = fn_state.currentstate._6;
        // D s_0_4: read-var descriptor:bv
        let s_0_4: Bits = fn_state.descriptor;
        // D s_0_5: call AArch64_LeafBase(s_0_4, s_0_0, s_0_1, s_0_2, s_0_3)
        let s_0_5: u64 = AArch64_LeafBase(
            state,
            tracer,
            s_0_4,
            s_0_0,
            s_0_1,
            s_0_2,
            s_0_3,
        );
        // D s_0_6: write-var baseaddress.0 <= s_0_5
        fn_state.baseaddress._0 = s_0_5;
        // D s_0_7: read-var currentstate.0:struct
        let s_0_7: ProductTypeda0231e9dc169f81 = fn_state.currentstate._0;
        // D s_0_8: write-var ga#13456 <= s_0_7
        fn_state.ga_13456 = s_0_7;
        // D s_0_9: read-var ga#13456.1:struct
        let s_0_9: u32 = fn_state.ga_13456._1;
        // C s_0_10: const #1u : u32
        let s_0_10: u32 = 1;
        // D s_0_11: cmp-eq s_0_9 s_0_10
        let s_0_11: bool = ((s_0_9) == (s_0_10));
        // N s_0_12: branch s_0_11 b72 b1
        if s_0_11 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_1_0: read-var currentstate.0:struct
        let s_1_0: ProductTypeda0231e9dc169f81 = fn_state.currentstate._0;
        // D s_1_1: write-var ga#13468 <= s_1_0
        fn_state.ga_13468 = s_1_0;
        // D s_1_2: read-var ga#13468.1:struct
        let s_1_2: u32 = fn_state.ga_13468._1;
        // C s_1_3: const #2u : u32
        let s_1_3: u32 = 2;
        // D s_1_4: cmp-eq s_1_2 s_1_3
        let s_1_4: bool = ((s_1_2) == (s_1_3));
        // N s_1_5: branch s_1_4 b62 b2
        if s_1_4 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_2_0: read-var currentstate.0:struct
        let s_2_0: ProductTypeda0231e9dc169f81 = fn_state.currentstate._0;
        // D s_2_1: write-var ga#13491 <= s_2_0
        fn_state.ga_13491 = s_2_0;
        // D s_2_2: read-var ga#13491.1:struct
        let s_2_2: u32 = fn_state.ga_13491._1;
        // C s_2_3: const #3u : u32
        let s_2_3: u32 = 3;
        // D s_2_4: cmp-eq s_2_2 s_2_3
        let s_2_4: bool = ((s_2_2) == (s_2_3));
        // N s_2_5: branch s_2_4 b58 b3
        if s_2_4 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#17975 <= s_3_0
        fn_state.gs_17975 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_4_0: read-var gs#17975:u8
        let s_4_0: bool = fn_state.gs_17975;
        // N s_4_1: branch s_4_0 b51 b5
        if s_4_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_5_0: read-var currentstate.0:struct
        let s_5_0: ProductTypeda0231e9dc169f81 = fn_state.currentstate._0;
        // D s_5_1: write-var ga#13505 <= s_5_0
        fn_state.ga_13505 = s_5_0;
        // D s_5_2: read-var ga#13505.1:struct
        let s_5_2: u32 = fn_state.ga_13505._1;
        // C s_5_3: const #3u : u32
        let s_5_3: u32 = 3;
        // D s_5_4: cmp-eq s_5_2 s_5_3
        let s_5_4: bool = ((s_5_2) == (s_5_3));
        // N s_5_5: branch s_5_4 b50 b6
        if s_5_4 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_6_0: const #0u : u32
        let s_6_0: u32 = 0;
        // D s_6_1: write-var baseaddress.1 <= s_6_0
        fn_state.baseaddress._1 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var nextstate.5 <= s_7_0
        fn_state.nextstate._5 = s_7_0;
        // D s_7_2: read-var currentstate.6:struct
        let s_7_2: i128 = fn_state.currentstate._6;
        // D s_7_3: write-var nextstate.6 <= s_7_2
        fn_state.nextstate._6 = s_7_2;
        // D s_7_4: read-var baseaddress:struct
        let s_7_4: ProductTypeda0231e9dc169f81 = fn_state.baseaddress;
        // D s_7_5: write-var nextstate.0 <= s_7_4
        fn_state.nextstate._0 = s_7_4;
        // D s_7_6: read-var walkparams.0:struct
        let s_7_6: bool = fn_state.walkparams._0;
        // D s_7_7: cast zx s_7_6 -> bv
        let s_7_7: Bits = Bits::new(s_7_6 as u128, 1u16);
        // C s_7_8: const #1u : u8
        let s_7_8: bool = true;
        // C s_7_9: cast zx s_7_8 -> bv
        let s_7_9: Bits = Bits::new(s_7_8 as u128, 1u16);
        // D s_7_10: cmp-eq s_7_7 s_7_9
        let s_7_10: bool = ((s_7_7) == (s_7_9));
        // N s_7_11: branch s_7_10 b47 b8
        if s_7_10 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_8_0: read-var descriptor:bv
        let s_8_0: Bits = fn_state.descriptor;
        // D s_8_1: size-of s_8_0
        let s_8_1: u16 = s_8_0.length();
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // C s_8_3: const #4s : i
        let s_8_3: i128 = 4;
        // D s_8_4: cmp-lt s_8_3 s_8_2
        let s_8_4: bool = ((s_8_3) < (s_8_2));
        // N s_8_5: assert s_8_4
        let s_8_5: () = assert!(s_8_4);
        // C s_8_6: const #2s : i
        let s_8_6: i128 = 2;
        // D s_8_7: read-var descriptor:bv
        let s_8_7: Bits = fn_state.descriptor;
        // C s_8_8: const #1s : i64
        let s_8_8: i64 = 1;
        // C s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // C s_8_10: const #2s : i
        let s_8_10: i128 = 2;
        // C s_8_11: add s_8_10 s_8_9
        let s_8_11: i128 = (s_8_10 + s_8_9);
        // D s_8_12: bit-extract s_8_7 s_8_6 s_8_11
        let s_8_12: Bits = (Bits::new(
            ((s_8_7) >> (s_8_6)).value(),
            u16::try_from(s_8_11).unwrap(),
        ));
        // D s_8_13: cast reint s_8_12 -> u8
        let s_8_13: u8 = (s_8_12.value() as u8);
        // C s_8_14: const #0u : u8
        let s_8_14: bool = false;
        // C s_8_15: cast zx s_8_14 -> bv
        let s_8_15: Bits = Bits::new(s_8_14 as u128, 1u16);
        // D s_8_16: cast zx s_8_13 -> bv
        let s_8_16: Bits = Bits::new(s_8_13 as u128, 3u16);
        // C s_8_17: cast reint s_8_15 -> u128
        let s_8_17: u128 = (s_8_15.value() as u128);
        // D s_8_18: size-of s_8_15
        let s_8_18: u16 = s_8_15.length();
        // D s_8_19: cast reint s_8_16 -> u128
        let s_8_19: u128 = (s_8_16.value() as u128);
        // D s_8_20: size-of s_8_16
        let s_8_20: u16 = s_8_16.length();
        // D s_8_21: lsl s_8_17 s_8_20
        let s_8_21: u128 = s_8_17 << s_8_20;
        // D s_8_22: or s_8_21 s_8_19
        let s_8_22: u128 = ((s_8_21) | (s_8_19));
        // D s_8_23: add s_8_18 s_8_20
        let s_8_23: u16 = (s_8_18 + s_8_20);
        // D s_8_24: create-bits s_8_22 s_8_23
        let s_8_24: Bits = Bits::new(s_8_22, s_8_23);
        // D s_8_25: cast reint s_8_24 -> u8
        let s_8_25: u8 = (s_8_24.value() as u8);
        // D s_8_26: write-var attrindx <= s_8_25
        fn_state.attrindx = s_8_25;
        // N s_8_27: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_9_0: read-var walkparams.3:struct
        let s_9_0: bool = fn_state.walkparams._3;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 1u16);
        // C s_9_2: const #1u : u8
        let s_9_2: bool = true;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // N s_9_5: branch s_9_4 b46 b10
        if s_9_4 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_10_0: read-var walkparams.7:struct
        let s_10_0: bool = fn_state.walkparams._7;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b45 b11
        if s_10_4 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_11_0: read-var descriptor:bv
        let s_11_0: Bits = fn_state.descriptor;
        // D s_11_1: size-of s_11_0
        let s_11_1: u16 = s_11_0.length();
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // C s_11_3: const #9s : i
        let s_11_3: i128 = 9;
        // D s_11_4: cmp-lt s_11_3 s_11_2
        let s_11_4: bool = ((s_11_3) < (s_11_2));
        // N s_11_5: assert s_11_4
        let s_11_5: () = assert!(s_11_4);
        // C s_11_6: const #8s : i
        let s_11_6: i128 = 8;
        // D s_11_7: read-var descriptor:bv
        let s_11_7: Bits = fn_state.descriptor;
        // C s_11_8: const #1s : i64
        let s_11_8: i64 = 1;
        // C s_11_9: cast zx s_11_8 -> i
        let s_11_9: i128 = (i128::try_from(s_11_8).unwrap());
        // C s_11_10: const #1s : i
        let s_11_10: i128 = 1;
        // C s_11_11: add s_11_10 s_11_9
        let s_11_11: i128 = (s_11_10 + s_11_9);
        // D s_11_12: bit-extract s_11_7 s_11_6 s_11_11
        let s_11_12: Bits = (Bits::new(
            ((s_11_7) >> (s_11_6)).value(),
            u16::try_from(s_11_11).unwrap(),
        ));
        // D s_11_13: cast reint s_11_12 -> u8
        let s_11_13: u8 = (s_11_12.value() as u8);
        // D s_11_14: write-var sh <= s_11_13
        fn_state.sh = s_11_13;
        // N s_11_15: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_12_0: read-var attrindx:u8
        let s_12_0: u8 = fn_state.attrindx;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 4u16);
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (s_12_1.value() as i128);
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // D s_12_4: read-var walkparams.18:struct
        let s_12_4: ProductType5c790c8ef59cc8b2 = fn_state.walkparams._18;
        // D s_12_5: read-var walkparams.17:struct
        let s_12_5: ProductType5c790c8ef59cc8b2 = fn_state.walkparams._17;
        // D s_12_6: cast zx s_12_3 -> i
        let s_12_6: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_7: call AArch64_MAIRAttr(s_12_6, s_12_4, s_12_5)
        let s_12_7: u8 = AArch64_MAIRAttr(state, tracer, s_12_6, s_12_4, s_12_5);
        // C s_12_8: const #1u : u8
        let s_12_8: bool = true;
        // D s_12_9: read-var sh:u8
        let s_12_9: u8 = fn_state.sh;
        // D s_12_10: read-var walkparams:struct
        let s_12_10: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_12_11: call S1DecodeMemAttrs(s_12_7, s_12_9, s_12_8, s_12_10)
        let s_12_11: ProductTypef170cab34335b70c = S1DecodeMemAttrs(
            state,
            tracer,
            s_12_7,
            s_12_9,
            s_12_8,
            s_12_10,
        );
        // D s_12_12: write-var nextstate.7 <= s_12_11
        fn_state.nextstate._7 = s_12_11;
        // D s_12_13: read-var currentstate.9:struct
        let s_12_13: ProductTypebf05c51f33174538 = fn_state.currentstate._9;
        // D s_12_14: read-var descriptor:bv
        let s_12_14: Bits = fn_state.descriptor;
        // D s_12_15: read-var regime:u32
        let s_12_15: u32 = fn_state.regime;
        // D s_12_16: read-var walkparams:struct
        let s_12_16: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_12_17: call AArch64_S1ApplyOutputPerms(s_12_13, s_12_14, s_12_15, s_12_16)
        let s_12_17: ProductTypebf05c51f33174538 = AArch64_S1ApplyOutputPerms(
            state,
            tracer,
            s_12_13,
            s_12_14,
            s_12_15,
            s_12_16,
        );
        // D s_12_18: write-var nextstate.9 <= s_12_17
        fn_state.nextstate._9 = s_12_17;
        // D s_12_19: read-var walkparams.3:struct
        let s_12_19: bool = fn_state.walkparams._3;
        // D s_12_20: cast zx s_12_19 -> bv
        let s_12_20: Bits = Bits::new(s_12_19 as u128, 1u16);
        // C s_12_21: const #1u : u8
        let s_12_21: bool = true;
        // C s_12_22: cast zx s_12_21 -> bv
        let s_12_22: Bits = Bits::new(s_12_21 as u128, 1u16);
        // D s_12_23: cmp-eq s_12_20 s_12_22
        let s_12_23: bool = ((s_12_20) == (s_12_22));
        // N s_12_24: branch s_12_23 b44 b13
        if s_12_23 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_13_0: read-var walkparams.27:struct
        let s_13_0: bool = fn_state.walkparams._27;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 1u16);
        // C s_13_2: const #1u : u8
        let s_13_2: bool = true;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // N s_13_5: branch s_13_4 b43 b14
        if s_13_4 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var protectedbit <= s_14_0
        fn_state.protectedbit = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // N s_15_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_16_0: read-var currentstate.10:struct
        let s_16_0: bool = fn_state.currentstate._10;
        // N s_16_1: branch s_16_0 b42 b17
        if s_16_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#18042 <= s_17_0
        fn_state.gs_18042 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_18_0: read-var gs#18042:u8
        let s_18_0: bool = fn_state.gs_18042;
        // N s_18_1: branch s_18_0 b41 b19
        if s_18_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#18043 <= s_19_0
        fn_state.gs_18043 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_20_0: read-var gs#18043:u8
        let s_20_0: bool = fn_state.gs_18043;
        // N s_20_1: branch s_20_0 b40 b21
        if s_20_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var nextstate.10 <= s_21_0
        fn_state.nextstate._10 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_22_0: read-var walkparams.27:struct
        let s_22_0: bool = fn_state.walkparams._27;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // C s_22_2: const #1u : u8
        let s_22_2: bool = true;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // N s_22_5: branch s_22_4 b39 b23
        if s_22_4 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_23_0: read-var currentstate.2:struct
        let s_23_0: bool = fn_state.currentstate._2;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 1u16);
        // C s_23_2: const #1u : u8
        let s_23_2: bool = true;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: write-var gs#18044 <= s_23_4
        fn_state.gs_18044 = s_23_4;
        // N s_23_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_24_0: read-var gs#18044:u8
        let s_24_0: bool = fn_state.gs_18044;
        // N s_24_1: branch s_24_0 b38 b25
        if s_24_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_25_0: read-var descriptor:bv
        let s_25_0: Bits = fn_state.descriptor;
        // D s_25_1: size-of s_25_0
        let s_25_1: u16 = s_25_0.length();
        // D s_25_2: cast zx s_25_1 -> i
        let s_25_2: i128 = (i128::try_from(s_25_1).unwrap());
        // C s_25_3: const #52s : i
        let s_25_3: i128 = 52;
        // D s_25_4: cmp-lt s_25_3 s_25_2
        let s_25_4: bool = ((s_25_3) < (s_25_2));
        // N s_25_5: assert s_25_4
        let s_25_5: () = assert!(s_25_4);
        // D s_25_6: read-var walkparams.36:struct
        let s_25_6: u32 = fn_state.walkparams._36;
        // D s_25_7: read-var walkparams.3:struct
        let s_25_7: bool = fn_state.walkparams._3;
        // D s_25_8: read-var currentstate.6:struct
        let s_25_8: i128 = fn_state.currentstate._6;
        // D s_25_9: read-var descriptor:bv
        let s_25_9: Bits = fn_state.descriptor;
        // D s_25_10: call AArch64_ContiguousBit(s_25_6, s_25_7, s_25_8, s_25_9)
        let s_25_10: bool = AArch64_ContiguousBit(
            state,
            tracer,
            s_25_6,
            s_25_7,
            s_25_8,
            s_25_9,
        );
        // D s_25_11: write-var nextstate.1 <= s_25_10
        fn_state.nextstate._1 = s_25_10;
        // N s_25_12: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_26_0: read-var regime:u32
        let s_26_0: u32 = fn_state.regime;
        // D s_26_1: call HasUnprivileged(s_26_0)
        let s_26_1: bool = HasUnprivileged(state, tracer, s_26_0);
        // D s_26_2: not s_26_1
        let s_26_2: bool = !s_26_1;
        // N s_26_3: branch s_26_2 b37 b27
        if s_26_2 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_27_0: read-var ss:u32
        let s_27_0: u32 = fn_state.ss;
        // C s_27_1: const #3u : u32
        let s_27_1: u32 = 3;
        // D s_27_2: cmp-eq s_27_0 s_27_1
        let s_27_2: bool = ((s_27_0) == (s_27_1));
        // N s_27_3: branch s_27_2 b36 b28
        if s_27_2 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#18055 <= s_28_0
        fn_state.gs_18055 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_29_0: read-var gs#18055:u8
        let s_29_0: bool = fn_state.gs_18055;
        // N s_29_1: branch s_29_0 b35 b30
        if s_29_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_30_0: read-var descriptor:bv
        let s_30_0: Bits = fn_state.descriptor;
        // D s_30_1: size-of s_30_0
        let s_30_1: u16 = s_30_0.length();
        // D s_30_2: cast zx s_30_1 -> i
        let s_30_2: i128 = (i128::try_from(s_30_1).unwrap());
        // C s_30_3: const #11s : i
        let s_30_3: i128 = 11;
        // D s_30_4: cmp-lt s_30_3 s_30_2
        let s_30_4: bool = ((s_30_3) < (s_30_2));
        // N s_30_5: assert s_30_4
        let s_30_5: () = assert!(s_30_4);
        // C s_30_6: const #11s : i
        let s_30_6: i128 = 11;
        // D s_30_7: read-var descriptor:bv
        let s_30_7: Bits = fn_state.descriptor;
        // C s_30_8: const #1u : u64
        let s_30_8: u64 = 1;
        // D s_30_9: bit-extract s_30_7 s_30_6 s_30_8
        let s_30_9: Bits = (Bits::new(
            ((s_30_7) >> (s_30_6)).value(),
            u16::try_from(s_30_8).unwrap(),
        ));
        // D s_30_10: cast reint s_30_9 -> u8
        let s_30_10: bool = ((s_30_9.value()) != 0);
        // C s_30_11: const #0s : i
        let s_30_11: i128 = 0;
        // C s_30_12: const #0u : u64
        let s_30_12: u64 = 0;
        // D s_30_13: cast zx s_30_10 -> u64
        let s_30_13: u64 = (s_30_10 as u64);
        // C s_30_14: const #1u : u64
        let s_30_14: u64 = 1;
        // D s_30_15: and s_30_13 s_30_14
        let s_30_15: u64 = ((s_30_13) & (s_30_14));
        // D s_30_16: cmp-eq s_30_15 s_30_14
        let s_30_16: bool = ((s_30_15) == (s_30_14));
        // D s_30_17: lsl s_30_13 s_30_11
        let s_30_17: u64 = s_30_13 << s_30_11;
        // D s_30_18: or s_30_12 s_30_17
        let s_30_18: u64 = ((s_30_12) | (s_30_17));
        // D s_30_19: cmpl s_30_17
        let s_30_19: u64 = !s_30_17;
        // D s_30_20: and s_30_12 s_30_19
        let s_30_20: u64 = ((s_30_12) & (s_30_19));
        // D s_30_21: select s_30_16 s_30_18 s_30_20
        let s_30_21: u64 = if s_30_16 { s_30_18 } else { s_30_20 };
        // D s_30_22: cast trunc s_30_21 -> u8
        let s_30_22: bool = ((s_30_21) != 0);
        // D s_30_23: write-var nextstate.8 <= s_30_22
        fn_state.nextstate._8 = s_30_22;
        // N s_30_24: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_31_0: read-var walkparams.3:struct
        let s_31_0: bool = fn_state.walkparams._3;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // C s_31_2: const #1u : u8
        let s_31_2: bool = true;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // N s_31_5: branch s_31_4 b34 b32
        if s_31_4 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_32_0: read-var descriptor:bv
        let s_32_0: Bits = fn_state.descriptor;
        // D s_32_1: size-of s_32_0
        let s_32_1: u16 = s_32_0.length();
        // D s_32_2: cast zx s_32_1 -> i
        let s_32_2: i128 = (i128::try_from(s_32_1).unwrap());
        // C s_32_3: const #50s : i
        let s_32_3: i128 = 50;
        // D s_32_4: cmp-lt s_32_3 s_32_2
        let s_32_4: bool = ((s_32_3) < (s_32_2));
        // N s_32_5: assert s_32_4
        let s_32_5: () = assert!(s_32_4);
        // C s_32_6: const #50s : i
        let s_32_6: i128 = 50;
        // D s_32_7: read-var descriptor:bv
        let s_32_7: Bits = fn_state.descriptor;
        // C s_32_8: const #1u : u64
        let s_32_8: u64 = 1;
        // D s_32_9: bit-extract s_32_7 s_32_6 s_32_8
        let s_32_9: Bits = (Bits::new(
            ((s_32_7) >> (s_32_6)).value(),
            u16::try_from(s_32_8).unwrap(),
        ));
        // D s_32_10: cast reint s_32_9 -> u8
        let s_32_10: bool = ((s_32_9.value()) != 0);
        // C s_32_11: const #0s : i
        let s_32_11: i128 = 0;
        // C s_32_12: const #0u : u64
        let s_32_12: u64 = 0;
        // D s_32_13: cast zx s_32_10 -> u64
        let s_32_13: u64 = (s_32_10 as u64);
        // C s_32_14: const #1u : u64
        let s_32_14: u64 = 1;
        // D s_32_15: and s_32_13 s_32_14
        let s_32_15: u64 = ((s_32_13) & (s_32_14));
        // D s_32_16: cmp-eq s_32_15 s_32_14
        let s_32_16: bool = ((s_32_15) == (s_32_14));
        // D s_32_17: lsl s_32_13 s_32_11
        let s_32_17: u64 = s_32_13 << s_32_11;
        // D s_32_18: or s_32_12 s_32_17
        let s_32_18: u64 = ((s_32_12) | (s_32_17));
        // D s_32_19: cmpl s_32_17
        let s_32_19: u64 = !s_32_17;
        // D s_32_20: and s_32_12 s_32_19
        let s_32_20: u64 = ((s_32_12) & (s_32_19));
        // D s_32_21: select s_32_16 s_32_18 s_32_20
        let s_32_21: u64 = if s_32_16 { s_32_18 } else { s_32_20 };
        // D s_32_22: cast trunc s_32_21 -> u8
        let s_32_22: bool = ((s_32_21) != 0);
        // D s_32_23: write-var nextstate.4 <= s_32_22
        fn_state.nextstate._4 = s_32_22;
        // N s_32_24: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_33_0: read-var nextstate:struct
        let s_33_0: ProductType96e7acababe246a1 = fn_state.nextstate;
        // N s_33_1: return s_33_0
        return s_33_0;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_34_0: read-var descriptor:bv
        let s_34_0: Bits = fn_state.descriptor;
        // D s_34_1: size-of s_34_0
        let s_34_1: u16 = s_34_0.length();
        // D s_34_2: cast zx s_34_1 -> i
        let s_34_2: i128 = (i128::try_from(s_34_1).unwrap());
        // C s_34_3: const #113s : i
        let s_34_3: i128 = 113;
        // D s_34_4: cmp-lt s_34_3 s_34_2
        let s_34_4: bool = ((s_34_3) < (s_34_2));
        // N s_34_5: assert s_34_4
        let s_34_5: () = assert!(s_34_4);
        // C s_34_6: const #113s : i
        let s_34_6: i128 = 113;
        // D s_34_7: read-var descriptor:bv
        let s_34_7: Bits = fn_state.descriptor;
        // C s_34_8: const #1u : u64
        let s_34_8: u64 = 1;
        // D s_34_9: bit-extract s_34_7 s_34_6 s_34_8
        let s_34_9: Bits = (Bits::new(
            ((s_34_7) >> (s_34_6)).value(),
            u16::try_from(s_34_8).unwrap(),
        ));
        // D s_34_10: cast reint s_34_9 -> u8
        let s_34_10: bool = ((s_34_9.value()) != 0);
        // C s_34_11: const #0s : i
        let s_34_11: i128 = 0;
        // C s_34_12: const #0u : u64
        let s_34_12: u64 = 0;
        // D s_34_13: cast zx s_34_10 -> u64
        let s_34_13: u64 = (s_34_10 as u64);
        // C s_34_14: const #1u : u64
        let s_34_14: u64 = 1;
        // D s_34_15: and s_34_13 s_34_14
        let s_34_15: u64 = ((s_34_13) & (s_34_14));
        // D s_34_16: cmp-eq s_34_15 s_34_14
        let s_34_16: bool = ((s_34_15) == (s_34_14));
        // D s_34_17: lsl s_34_13 s_34_11
        let s_34_17: u64 = s_34_13 << s_34_11;
        // D s_34_18: or s_34_12 s_34_17
        let s_34_18: u64 = ((s_34_12) | (s_34_17));
        // D s_34_19: cmpl s_34_17
        let s_34_19: u64 = !s_34_17;
        // D s_34_20: and s_34_12 s_34_19
        let s_34_20: u64 = ((s_34_12) & (s_34_19));
        // D s_34_21: select s_34_16 s_34_18 s_34_20
        let s_34_21: u64 = if s_34_16 { s_34_18 } else { s_34_20 };
        // D s_34_22: cast trunc s_34_21 -> u8
        let s_34_22: bool = ((s_34_21) != 0);
        // D s_34_23: write-var nextstate.4 <= s_34_22
        fn_state.nextstate._4 = s_34_22;
        // N s_34_24: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var nextstate.8 <= s_35_0
        fn_state.nextstate._8 = s_35_0;
        // N s_35_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_36_0: read-var currentstate.0:struct
        let s_36_0: ProductTypeda0231e9dc169f81 = fn_state.currentstate._0;
        // D s_36_1: write-var ga#13555 <= s_36_0
        fn_state.ga_13555 = s_36_0;
        // D s_36_2: read-var ga#13555.1:struct
        let s_36_2: u32 = fn_state.ga_13555._1;
        // C s_36_3: const #0u : u32
        let s_36_3: u32 = 0;
        // D s_36_4: cmp-eq s_36_2 s_36_3
        let s_36_4: bool = ((s_36_2) == (s_36_3));
        // D s_36_5: write-var gs#18055 <= s_36_4
        fn_state.gs_18055 = s_36_4;
        // N s_36_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var nextstate.8 <= s_37_0
        fn_state.nextstate._8 = s_37_0;
        // N s_37_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var nextstate.1 <= s_38_0
        fn_state.nextstate._1 = s_38_0;
        // N s_38_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#18044 <= s_39_0
        fn_state.gs_18044 = s_39_0;
        // N s_39_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // D s_40_1: write-var nextstate.10 <= s_40_0
        fn_state.nextstate._10 = s_40_0;
        // N s_40_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_41_0: read-var protectedbit:u8
        let s_41_0: bool = fn_state.protectedbit;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 1u16);
        // C s_41_2: const #1u : u8
        let s_41_2: bool = true;
        // C s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // D s_41_4: cmp-eq s_41_1 s_41_3
        let s_41_4: bool = ((s_41_1) == (s_41_3));
        // D s_41_5: write-var gs#18043 <= s_41_4
        fn_state.gs_18043 = s_41_4;
        // N s_41_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_42_0: read-var s2fs1mro:u8
        let s_42_0: bool = fn_state.s2fs1mro;
        // D s_42_1: write-var gs#18042 <= s_42_0
        fn_state.gs_18042 = s_42_0;
        // N s_42_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_43_0: read-var descriptor:bv
        let s_43_0: Bits = fn_state.descriptor;
        // D s_43_1: size-of s_43_0
        let s_43_1: u16 = s_43_0.length();
        // D s_43_2: cast zx s_43_1 -> i
        let s_43_2: i128 = (i128::try_from(s_43_1).unwrap());
        // C s_43_3: const #52s : i
        let s_43_3: i128 = 52;
        // D s_43_4: cmp-lt s_43_3 s_43_2
        let s_43_4: bool = ((s_43_3) < (s_43_2));
        // N s_43_5: assert s_43_4
        let s_43_5: () = assert!(s_43_4);
        // C s_43_6: const #52s : i
        let s_43_6: i128 = 52;
        // D s_43_7: read-var descriptor:bv
        let s_43_7: Bits = fn_state.descriptor;
        // C s_43_8: const #1u : u64
        let s_43_8: u64 = 1;
        // D s_43_9: bit-extract s_43_7 s_43_6 s_43_8
        let s_43_9: Bits = (Bits::new(
            ((s_43_7) >> (s_43_6)).value(),
            u16::try_from(s_43_8).unwrap(),
        ));
        // D s_43_10: cast reint s_43_9 -> u8
        let s_43_10: bool = ((s_43_9.value()) != 0);
        // C s_43_11: const #0s : i
        let s_43_11: i128 = 0;
        // C s_43_12: const #0u : u64
        let s_43_12: u64 = 0;
        // D s_43_13: cast zx s_43_10 -> u64
        let s_43_13: u64 = (s_43_10 as u64);
        // C s_43_14: const #1u : u64
        let s_43_14: u64 = 1;
        // D s_43_15: and s_43_13 s_43_14
        let s_43_15: u64 = ((s_43_13) & (s_43_14));
        // D s_43_16: cmp-eq s_43_15 s_43_14
        let s_43_16: bool = ((s_43_15) == (s_43_14));
        // D s_43_17: lsl s_43_13 s_43_11
        let s_43_17: u64 = s_43_13 << s_43_11;
        // D s_43_18: or s_43_12 s_43_17
        let s_43_18: u64 = ((s_43_12) | (s_43_17));
        // D s_43_19: cmpl s_43_17
        let s_43_19: u64 = !s_43_17;
        // D s_43_20: and s_43_12 s_43_19
        let s_43_20: u64 = ((s_43_12) & (s_43_19));
        // D s_43_21: select s_43_16 s_43_18 s_43_20
        let s_43_21: u64 = if s_43_16 { s_43_18 } else { s_43_20 };
        // D s_43_22: cast trunc s_43_21 -> u8
        let s_43_22: bool = ((s_43_21) != 0);
        // D s_43_23: write-var protectedbit <= s_43_22
        fn_state.protectedbit = s_43_22;
        // N s_43_24: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_44_0: read-var descriptor:bv
        let s_44_0: Bits = fn_state.descriptor;
        // D s_44_1: size-of s_44_0
        let s_44_1: u16 = s_44_0.length();
        // D s_44_2: cast zx s_44_1 -> i
        let s_44_2: i128 = (i128::try_from(s_44_1).unwrap());
        // C s_44_3: const #114s : i
        let s_44_3: i128 = 114;
        // D s_44_4: cmp-lt s_44_3 s_44_2
        let s_44_4: bool = ((s_44_3) < (s_44_2));
        // N s_44_5: assert s_44_4
        let s_44_5: () = assert!(s_44_4);
        // C s_44_6: const #114s : i
        let s_44_6: i128 = 114;
        // D s_44_7: read-var descriptor:bv
        let s_44_7: Bits = fn_state.descriptor;
        // C s_44_8: const #1u : u64
        let s_44_8: u64 = 1;
        // D s_44_9: bit-extract s_44_7 s_44_6 s_44_8
        let s_44_9: Bits = (Bits::new(
            ((s_44_7) >> (s_44_6)).value(),
            u16::try_from(s_44_8).unwrap(),
        ));
        // D s_44_10: cast reint s_44_9 -> u8
        let s_44_10: bool = ((s_44_9.value()) != 0);
        // C s_44_11: const #0s : i
        let s_44_11: i128 = 0;
        // C s_44_12: const #0u : u64
        let s_44_12: u64 = 0;
        // D s_44_13: cast zx s_44_10 -> u64
        let s_44_13: u64 = (s_44_10 as u64);
        // C s_44_14: const #1u : u64
        let s_44_14: u64 = 1;
        // D s_44_15: and s_44_13 s_44_14
        let s_44_15: u64 = ((s_44_13) & (s_44_14));
        // D s_44_16: cmp-eq s_44_15 s_44_14
        let s_44_16: bool = ((s_44_15) == (s_44_14));
        // D s_44_17: lsl s_44_13 s_44_11
        let s_44_17: u64 = s_44_13 << s_44_11;
        // D s_44_18: or s_44_12 s_44_17
        let s_44_18: u64 = ((s_44_12) | (s_44_17));
        // D s_44_19: cmpl s_44_17
        let s_44_19: u64 = !s_44_17;
        // D s_44_20: and s_44_12 s_44_19
        let s_44_20: u64 = ((s_44_12) & (s_44_19));
        // D s_44_21: select s_44_16 s_44_18 s_44_20
        let s_44_21: u64 = if s_44_16 { s_44_18 } else { s_44_20 };
        // D s_44_22: cast trunc s_44_21 -> u8
        let s_44_22: bool = ((s_44_21) != 0);
        // D s_44_23: write-var protectedbit <= s_44_22
        fn_state.protectedbit = s_44_22;
        // N s_44_24: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_45_0: read-var walkparams.29:struct
        let s_45_0: u8 = fn_state.walkparams._29;
        // D s_45_1: write-var sh <= s_45_0
        fn_state.sh = s_45_0;
        // N s_45_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_46_0: read-var descriptor:bv
        let s_46_0: Bits = fn_state.descriptor;
        // D s_46_1: size-of s_46_0
        let s_46_1: u16 = s_46_0.length();
        // D s_46_2: cast zx s_46_1 -> i
        let s_46_2: i128 = (i128::try_from(s_46_1).unwrap());
        // C s_46_3: const #9s : i
        let s_46_3: i128 = 9;
        // D s_46_4: cmp-lt s_46_3 s_46_2
        let s_46_4: bool = ((s_46_3) < (s_46_2));
        // N s_46_5: assert s_46_4
        let s_46_5: () = assert!(s_46_4);
        // C s_46_6: const #8s : i
        let s_46_6: i128 = 8;
        // D s_46_7: read-var descriptor:bv
        let s_46_7: Bits = fn_state.descriptor;
        // C s_46_8: const #1s : i64
        let s_46_8: i64 = 1;
        // C s_46_9: cast zx s_46_8 -> i
        let s_46_9: i128 = (i128::try_from(s_46_8).unwrap());
        // C s_46_10: const #1s : i
        let s_46_10: i128 = 1;
        // C s_46_11: add s_46_10 s_46_9
        let s_46_11: i128 = (s_46_10 + s_46_9);
        // D s_46_12: bit-extract s_46_7 s_46_6 s_46_11
        let s_46_12: Bits = (Bits::new(
            ((s_46_7) >> (s_46_6)).value(),
            u16::try_from(s_46_11).unwrap(),
        ));
        // D s_46_13: cast reint s_46_12 -> u8
        let s_46_13: u8 = (s_46_12.value() as u8);
        // D s_46_14: write-var sh <= s_46_13
        fn_state.sh = s_46_13;
        // N s_46_15: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_47_0: read-var walkparams.3:struct
        let s_47_0: bool = fn_state.walkparams._3;
        // D s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 1u16);
        // C s_47_2: const #1u : u8
        let s_47_2: bool = true;
        // C s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // D s_47_4: cmp-eq s_47_1 s_47_3
        let s_47_4: bool = ((s_47_1) == (s_47_3));
        // N s_47_5: branch s_47_4 b49 b48
        if s_47_4 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_48_0: read-var descriptor:bv
        let s_48_0: Bits = fn_state.descriptor;
        // D s_48_1: size-of s_48_0
        let s_48_1: u16 = s_48_0.length();
        // D s_48_2: cast zx s_48_1 -> i
        let s_48_2: i128 = (i128::try_from(s_48_1).unwrap());
        // C s_48_3: const #59s : i
        let s_48_3: i128 = 59;
        // D s_48_4: cmp-lt s_48_3 s_48_2
        let s_48_4: bool = ((s_48_3) < (s_48_2));
        // N s_48_5: assert s_48_4
        let s_48_5: () = assert!(s_48_4);
        // C s_48_6: const #59s : i
        let s_48_6: i128 = 59;
        // D s_48_7: read-var descriptor:bv
        let s_48_7: Bits = fn_state.descriptor;
        // C s_48_8: const #1u : u64
        let s_48_8: u64 = 1;
        // D s_48_9: bit-extract s_48_7 s_48_6 s_48_8
        let s_48_9: Bits = (Bits::new(
            ((s_48_7) >> (s_48_6)).value(),
            u16::try_from(s_48_8).unwrap(),
        ));
        // D s_48_10: cast reint s_48_9 -> u8
        let s_48_10: bool = ((s_48_9.value()) != 0);
        // C s_48_11: const #0s : i
        let s_48_11: i128 = 0;
        // C s_48_12: const #0u : u64
        let s_48_12: u64 = 0;
        // D s_48_13: cast zx s_48_10 -> u64
        let s_48_13: u64 = (s_48_10 as u64);
        // C s_48_14: const #1u : u64
        let s_48_14: u64 = 1;
        // D s_48_15: and s_48_13 s_48_14
        let s_48_15: u64 = ((s_48_13) & (s_48_14));
        // D s_48_16: cmp-eq s_48_15 s_48_14
        let s_48_16: bool = ((s_48_15) == (s_48_14));
        // D s_48_17: lsl s_48_13 s_48_11
        let s_48_17: u64 = s_48_13 << s_48_11;
        // D s_48_18: or s_48_12 s_48_17
        let s_48_18: u64 = ((s_48_12) | (s_48_17));
        // D s_48_19: cmpl s_48_17
        let s_48_19: u64 = !s_48_17;
        // D s_48_20: and s_48_12 s_48_19
        let s_48_20: u64 = ((s_48_12) & (s_48_19));
        // D s_48_21: select s_48_16 s_48_18 s_48_20
        let s_48_21: u64 = if s_48_16 { s_48_18 } else { s_48_20 };
        // D s_48_22: cast trunc s_48_21 -> u8
        let s_48_22: bool = ((s_48_21) != 0);
        // C s_48_23: const #2s : i
        let s_48_23: i128 = 2;
        // D s_48_24: read-var descriptor:bv
        let s_48_24: Bits = fn_state.descriptor;
        // C s_48_25: const #1s : i64
        let s_48_25: i64 = 1;
        // C s_48_26: cast zx s_48_25 -> i
        let s_48_26: i128 = (i128::try_from(s_48_25).unwrap());
        // C s_48_27: const #2s : i
        let s_48_27: i128 = 2;
        // C s_48_28: add s_48_27 s_48_26
        let s_48_28: i128 = (s_48_27 + s_48_26);
        // D s_48_29: bit-extract s_48_24 s_48_23 s_48_28
        let s_48_29: Bits = (Bits::new(
            ((s_48_24) >> (s_48_23)).value(),
            u16::try_from(s_48_28).unwrap(),
        ));
        // D s_48_30: cast reint s_48_29 -> u8
        let s_48_30: u8 = (s_48_29.value() as u8);
        // D s_48_31: cast zx s_48_22 -> bv
        let s_48_31: Bits = Bits::new(s_48_22 as u128, 1u16);
        // D s_48_32: cast zx s_48_30 -> bv
        let s_48_32: Bits = Bits::new(s_48_30 as u128, 3u16);
        // D s_48_33: cast reint s_48_31 -> u128
        let s_48_33: u128 = (s_48_31.value() as u128);
        // D s_48_34: size-of s_48_31
        let s_48_34: u16 = s_48_31.length();
        // D s_48_35: cast reint s_48_32 -> u128
        let s_48_35: u128 = (s_48_32.value() as u128);
        // D s_48_36: size-of s_48_32
        let s_48_36: u16 = s_48_32.length();
        // D s_48_37: lsl s_48_33 s_48_36
        let s_48_37: u128 = s_48_33 << s_48_36;
        // D s_48_38: or s_48_37 s_48_35
        let s_48_38: u128 = ((s_48_37) | (s_48_35));
        // D s_48_39: add s_48_34 s_48_36
        let s_48_39: u16 = (s_48_34 + s_48_36);
        // D s_48_40: create-bits s_48_38 s_48_39
        let s_48_40: Bits = Bits::new(s_48_38, s_48_39);
        // D s_48_41: cast reint s_48_40 -> u8
        let s_48_41: u8 = (s_48_40.value() as u8);
        // D s_48_42: write-var attrindx <= s_48_41
        fn_state.attrindx = s_48_41;
        // N s_48_43: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_49_0: read-var descriptor:bv
        let s_49_0: Bits = fn_state.descriptor;
        // D s_49_1: size-of s_49_0
        let s_49_1: u16 = s_49_0.length();
        // D s_49_2: cast zx s_49_1 -> i
        let s_49_2: i128 = (i128::try_from(s_49_1).unwrap());
        // C s_49_3: const #5s : i
        let s_49_3: i128 = 5;
        // D s_49_4: cmp-lt s_49_3 s_49_2
        let s_49_4: bool = ((s_49_3) < (s_49_2));
        // N s_49_5: assert s_49_4
        let s_49_5: () = assert!(s_49_4);
        // C s_49_6: const #2s : i
        let s_49_6: i128 = 2;
        // D s_49_7: read-var descriptor:bv
        let s_49_7: Bits = fn_state.descriptor;
        // C s_49_8: const #1s : i64
        let s_49_8: i64 = 1;
        // C s_49_9: cast zx s_49_8 -> i
        let s_49_9: i128 = (i128::try_from(s_49_8).unwrap());
        // C s_49_10: const #3s : i
        let s_49_10: i128 = 3;
        // C s_49_11: add s_49_10 s_49_9
        let s_49_11: i128 = (s_49_10 + s_49_9);
        // D s_49_12: bit-extract s_49_7 s_49_6 s_49_11
        let s_49_12: Bits = (Bits::new(
            ((s_49_7) >> (s_49_6)).value(),
            u16::try_from(s_49_11).unwrap(),
        ));
        // D s_49_13: cast reint s_49_12 -> u8
        let s_49_13: u8 = (s_49_12.value() as u8);
        // D s_49_14: write-var attrindx <= s_49_13
        fn_state.attrindx = s_49_13;
        // N s_49_15: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_50_0: const #3u : u32
        let s_50_0: u32 = 3;
        // D s_50_1: write-var baseaddress.1 <= s_50_0
        fn_state.baseaddress._1 = s_50_0;
        // N s_50_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_51_0: read-var walkparams.3:struct
        let s_51_0: bool = fn_state.walkparams._3;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 1u16);
        // C s_51_2: const #1u : u8
        let s_51_2: bool = true;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // N s_51_5: branch s_51_4 b57 b52
        if s_51_4 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_52_0: read-var descriptor:bv
        let s_52_0: Bits = fn_state.descriptor;
        // D s_52_1: size-of s_52_0
        let s_52_1: u16 = s_52_0.length();
        // D s_52_2: cast zx s_52_1 -> i
        let s_52_2: i128 = (i128::try_from(s_52_1).unwrap());
        // C s_52_3: const #5s : i
        let s_52_3: i128 = 5;
        // D s_52_4: cmp-lt s_52_3 s_52_2
        let s_52_4: bool = ((s_52_3) < (s_52_2));
        // N s_52_5: assert s_52_4
        let s_52_5: () = assert!(s_52_4);
        // C s_52_6: const #5s : i
        let s_52_6: i128 = 5;
        // D s_52_7: read-var descriptor:bv
        let s_52_7: Bits = fn_state.descriptor;
        // C s_52_8: const #1u : u64
        let s_52_8: u64 = 1;
        // D s_52_9: bit-extract s_52_7 s_52_6 s_52_8
        let s_52_9: Bits = (Bits::new(
            ((s_52_7) >> (s_52_6)).value(),
            u16::try_from(s_52_8).unwrap(),
        ));
        // D s_52_10: cast reint s_52_9 -> u8
        let s_52_10: bool = ((s_52_9.value()) != 0);
        // C s_52_11: const #0s : i
        let s_52_11: i128 = 0;
        // C s_52_12: const #0u : u64
        let s_52_12: u64 = 0;
        // D s_52_13: cast zx s_52_10 -> u64
        let s_52_13: u64 = (s_52_10 as u64);
        // C s_52_14: const #1u : u64
        let s_52_14: u64 = 1;
        // D s_52_15: and s_52_13 s_52_14
        let s_52_15: u64 = ((s_52_13) & (s_52_14));
        // D s_52_16: cmp-eq s_52_15 s_52_14
        let s_52_16: bool = ((s_52_15) == (s_52_14));
        // D s_52_17: lsl s_52_13 s_52_11
        let s_52_17: u64 = s_52_13 << s_52_11;
        // D s_52_18: or s_52_12 s_52_17
        let s_52_18: u64 = ((s_52_12) | (s_52_17));
        // D s_52_19: cmpl s_52_17
        let s_52_19: u64 = !s_52_17;
        // D s_52_20: and s_52_12 s_52_19
        let s_52_20: u64 = ((s_52_12) & (s_52_19));
        // D s_52_21: select s_52_16 s_52_18 s_52_20
        let s_52_21: u64 = if s_52_16 { s_52_18 } else { s_52_20 };
        // D s_52_22: cast trunc s_52_21 -> u8
        let s_52_22: bool = ((s_52_21) != 0);
        // D s_52_23: write-var u#675 <= s_52_22
        fn_state.u_675 = s_52_22;
        // N s_52_24: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_53_0: read-var u#675:u8
        let s_53_0: bool = fn_state.u_675;
        // D s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 1u16);
        // C s_53_2: const #0u : u8
        let s_53_2: bool = false;
        // C s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // D s_53_4: cmp-eq s_53_1 s_53_3
        let s_53_4: bool = ((s_53_1) == (s_53_3));
        // N s_53_5: branch s_53_4 b56 b54
        if s_53_4 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_54_0: const #0u : u32
        let s_54_0: u32 = 0;
        // D s_54_1: write-var baseaddress.1 <= s_54_0
        fn_state.baseaddress._1 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // N s_55_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_56_0: const #3u : u32
        let s_56_0: u32 = 3;
        // D s_56_1: write-var baseaddress.1 <= s_56_0
        fn_state.baseaddress._1 = s_56_0;
        // N s_56_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_57_0: read-var descriptor:bv
        let s_57_0: Bits = fn_state.descriptor;
        // D s_57_1: size-of s_57_0
        let s_57_1: u16 = s_57_0.length();
        // D s_57_2: cast zx s_57_1 -> i
        let s_57_2: i128 = (i128::try_from(s_57_1).unwrap());
        // C s_57_3: const #127s : i
        let s_57_3: i128 = 127;
        // D s_57_4: cmp-lt s_57_3 s_57_2
        let s_57_4: bool = ((s_57_3) < (s_57_2));
        // N s_57_5: assert s_57_4
        let s_57_5: () = assert!(s_57_4);
        // C s_57_6: const #127s : i
        let s_57_6: i128 = 127;
        // D s_57_7: read-var descriptor:bv
        let s_57_7: Bits = fn_state.descriptor;
        // C s_57_8: const #1u : u64
        let s_57_8: u64 = 1;
        // D s_57_9: bit-extract s_57_7 s_57_6 s_57_8
        let s_57_9: Bits = (Bits::new(
            ((s_57_7) >> (s_57_6)).value(),
            u16::try_from(s_57_8).unwrap(),
        ));
        // D s_57_10: cast reint s_57_9 -> u8
        let s_57_10: bool = ((s_57_9.value()) != 0);
        // C s_57_11: const #0s : i
        let s_57_11: i128 = 0;
        // C s_57_12: const #0u : u64
        let s_57_12: u64 = 0;
        // D s_57_13: cast zx s_57_10 -> u64
        let s_57_13: u64 = (s_57_10 as u64);
        // C s_57_14: const #1u : u64
        let s_57_14: u64 = 1;
        // D s_57_15: and s_57_13 s_57_14
        let s_57_15: u64 = ((s_57_13) & (s_57_14));
        // D s_57_16: cmp-eq s_57_15 s_57_14
        let s_57_16: bool = ((s_57_15) == (s_57_14));
        // D s_57_17: lsl s_57_13 s_57_11
        let s_57_17: u64 = s_57_13 << s_57_11;
        // D s_57_18: or s_57_12 s_57_17
        let s_57_18: u64 = ((s_57_12) | (s_57_17));
        // D s_57_19: cmpl s_57_17
        let s_57_19: u64 = !s_57_17;
        // D s_57_20: and s_57_12 s_57_19
        let s_57_20: u64 = ((s_57_12) & (s_57_19));
        // D s_57_21: select s_57_16 s_57_18 s_57_20
        let s_57_21: u64 = if s_57_16 { s_57_18 } else { s_57_20 };
        // D s_57_22: cast trunc s_57_21 -> u8
        let s_57_22: bool = ((s_57_21) != 0);
        // D s_57_23: write-var u#675 <= s_57_22
        fn_state.u_675 = s_57_22;
        // N s_57_24: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_58_0: read-var regime:u32
        let s_58_0: u32 = fn_state.regime;
        // C s_58_1: const #2u : u32
        let s_58_1: u32 = 2;
        // D s_58_2: cmp-eq s_58_0 s_58_1
        let s_58_2: bool = ((s_58_0) == (s_58_1));
        // N s_58_3: branch s_58_2 b61 b59
        if s_58_2 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_59_0: read-var regime:u32
        let s_59_0: u32 = fn_state.regime;
        // C s_59_1: const #3u : u32
        let s_59_1: u32 = 3;
        // D s_59_2: cmp-eq s_59_0 s_59_1
        let s_59_2: bool = ((s_59_0) == (s_59_1));
        // D s_59_3: write-var gs#17974 <= s_59_2
        fn_state.gs_17974 = s_59_2;
        // N s_59_4: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_60_0: read-var gs#17974:u8
        let s_60_0: bool = fn_state.gs_17974;
        // D s_60_1: write-var gs#17975 <= s_60_0
        fn_state.gs_17975 = s_60_0;
        // N s_60_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var gs#17974 <= s_61_0
        fn_state.gs_17974 = s_61_0;
        // N s_61_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_62_0: read-var walkparams.3:struct
        let s_62_0: bool = fn_state.walkparams._3;
        // D s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 1u16);
        // C s_62_2: const #1u : u8
        let s_62_2: bool = true;
        // C s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_4: cmp-eq s_62_1 s_62_3
        let s_62_4: bool = ((s_62_1) == (s_62_3));
        // N s_62_5: branch s_62_4 b71 b63
        if s_62_4 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_63_0: read-var descriptor:bv
        let s_63_0: Bits = fn_state.descriptor;
        // D s_63_1: size-of s_63_0
        let s_63_1: u16 = s_63_0.length();
        // D s_63_2: cast zx s_63_1 -> i
        let s_63_2: i128 = (i128::try_from(s_63_1).unwrap());
        // C s_63_3: const #11s : i
        let s_63_3: i128 = 11;
        // D s_63_4: cmp-lt s_63_3 s_63_2
        let s_63_4: bool = ((s_63_3) < (s_63_2));
        // N s_63_5: assert s_63_4
        let s_63_5: () = assert!(s_63_4);
        // C s_63_6: const #11s : i
        let s_63_6: i128 = 11;
        // D s_63_7: read-var descriptor:bv
        let s_63_7: Bits = fn_state.descriptor;
        // C s_63_8: const #1u : u64
        let s_63_8: u64 = 1;
        // D s_63_9: bit-extract s_63_7 s_63_6 s_63_8
        let s_63_9: Bits = (Bits::new(
            ((s_63_7) >> (s_63_6)).value(),
            u16::try_from(s_63_8).unwrap(),
        ));
        // D s_63_10: cast reint s_63_9 -> u8
        let s_63_10: bool = ((s_63_9.value()) != 0);
        // C s_63_11: const #0s : i
        let s_63_11: i128 = 0;
        // C s_63_12: const #0u : u64
        let s_63_12: u64 = 0;
        // D s_63_13: cast zx s_63_10 -> u64
        let s_63_13: u64 = (s_63_10 as u64);
        // C s_63_14: const #1u : u64
        let s_63_14: u64 = 1;
        // D s_63_15: and s_63_13 s_63_14
        let s_63_15: u64 = ((s_63_13) & (s_63_14));
        // D s_63_16: cmp-eq s_63_15 s_63_14
        let s_63_16: bool = ((s_63_15) == (s_63_14));
        // D s_63_17: lsl s_63_13 s_63_11
        let s_63_17: u64 = s_63_13 << s_63_11;
        // D s_63_18: or s_63_12 s_63_17
        let s_63_18: u64 = ((s_63_12) | (s_63_17));
        // D s_63_19: cmpl s_63_17
        let s_63_19: u64 = !s_63_17;
        // D s_63_20: and s_63_12 s_63_19
        let s_63_20: u64 = ((s_63_12) & (s_63_19));
        // D s_63_21: select s_63_16 s_63_18 s_63_20
        let s_63_21: u64 = if s_63_16 { s_63_18 } else { s_63_20 };
        // D s_63_22: cast trunc s_63_21 -> u8
        let s_63_22: bool = ((s_63_21) != 0);
        // C s_63_23: const #5s : i
        let s_63_23: i128 = 5;
        // D s_63_24: read-var descriptor:bv
        let s_63_24: Bits = fn_state.descriptor;
        // C s_63_25: const #1u : u64
        let s_63_25: u64 = 1;
        // D s_63_26: bit-extract s_63_24 s_63_23 s_63_25
        let s_63_26: Bits = (Bits::new(
            ((s_63_24) >> (s_63_23)).value(),
            u16::try_from(s_63_25).unwrap(),
        ));
        // D s_63_27: cast reint s_63_26 -> u8
        let s_63_27: bool = ((s_63_26.value()) != 0);
        // C s_63_28: const #0s : i
        let s_63_28: i128 = 0;
        // C s_63_29: const #0u : u64
        let s_63_29: u64 = 0;
        // D s_63_30: cast zx s_63_27 -> u64
        let s_63_30: u64 = (s_63_27 as u64);
        // C s_63_31: const #1u : u64
        let s_63_31: u64 = 1;
        // D s_63_32: and s_63_30 s_63_31
        let s_63_32: u64 = ((s_63_30) & (s_63_31));
        // D s_63_33: cmp-eq s_63_32 s_63_31
        let s_63_33: bool = ((s_63_32) == (s_63_31));
        // D s_63_34: lsl s_63_30 s_63_28
        let s_63_34: u64 = s_63_30 << s_63_28;
        // D s_63_35: or s_63_29 s_63_34
        let s_63_35: u64 = ((s_63_29) | (s_63_34));
        // D s_63_36: cmpl s_63_34
        let s_63_36: u64 = !s_63_34;
        // D s_63_37: and s_63_29 s_63_36
        let s_63_37: u64 = ((s_63_29) & (s_63_36));
        // D s_63_38: select s_63_33 s_63_35 s_63_37
        let s_63_38: u64 = if s_63_33 { s_63_35 } else { s_63_37 };
        // D s_63_39: cast trunc s_63_38 -> u8
        let s_63_39: bool = ((s_63_38) != 0);
        // D s_63_40: cast zx s_63_22 -> bv
        let s_63_40: Bits = Bits::new(s_63_22 as u128, 1u16);
        // D s_63_41: cast zx s_63_39 -> bv
        let s_63_41: Bits = Bits::new(s_63_39 as u128, 1u16);
        // D s_63_42: cast reint s_63_40 -> u128
        let s_63_42: u128 = (s_63_40.value() as u128);
        // D s_63_43: size-of s_63_40
        let s_63_43: u16 = s_63_40.length();
        // D s_63_44: cast reint s_63_41 -> u128
        let s_63_44: u128 = (s_63_41.value() as u128);
        // D s_63_45: size-of s_63_41
        let s_63_45: u16 = s_63_41.length();
        // D s_63_46: lsl s_63_42 s_63_45
        let s_63_46: u128 = s_63_42 << s_63_45;
        // D s_63_47: or s_63_46 s_63_44
        let s_63_47: u128 = ((s_63_46) | (s_63_44));
        // D s_63_48: add s_63_43 s_63_45
        let s_63_48: u16 = (s_63_43 + s_63_45);
        // D s_63_49: create-bits s_63_47 s_63_48
        let s_63_49: Bits = Bits::new(s_63_47, s_63_48);
        // D s_63_50: cast reint s_63_49 -> u8
        let s_63_50: u8 = (s_63_49.value() as u8);
        // D s_63_51: write-var split_vec <= s_63_50
        fn_state.split_vec = s_63_50;
        // N s_63_52: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_64_0: const #1s : i
        let s_64_0: i128 = 1;
        // D s_64_1: read-var split_vec:u8
        let s_64_1: u8 = fn_state.split_vec;
        // D s_64_2: cast zx s_64_1 -> bv
        let s_64_2: Bits = Bits::new(s_64_1 as u128, 2u16);
        // C s_64_3: const #1s : i64
        let s_64_3: i64 = 1;
        // C s_64_4: cast zx s_64_3 -> i
        let s_64_4: i128 = (i128::try_from(s_64_3).unwrap());
        // C s_64_5: const #0s : i
        let s_64_5: i128 = 0;
        // C s_64_6: add s_64_5 s_64_4
        let s_64_6: i128 = (s_64_5 + s_64_4);
        // D s_64_7: bit-extract s_64_2 s_64_0 s_64_6
        let s_64_7: Bits = (Bits::new(
            ((s_64_2) >> (s_64_0)).value(),
            u16::try_from(s_64_6).unwrap(),
        ));
        // D s_64_8: cast reint s_64_7 -> u8
        let s_64_8: bool = ((s_64_7.value()) != 0);
        // C s_64_9: const #0s : i
        let s_64_9: i128 = 0;
        // D s_64_10: read-var split_vec:u8
        let s_64_10: u8 = fn_state.split_vec;
        // D s_64_11: cast zx s_64_10 -> bv
        let s_64_11: Bits = Bits::new(s_64_10 as u128, 2u16);
        // C s_64_12: const #1s : i64
        let s_64_12: i64 = 1;
        // C s_64_13: cast zx s_64_12 -> i
        let s_64_13: i128 = (i128::try_from(s_64_12).unwrap());
        // C s_64_14: const #0s : i
        let s_64_14: i128 = 0;
        // C s_64_15: add s_64_14 s_64_13
        let s_64_15: i128 = (s_64_14 + s_64_13);
        // D s_64_16: bit-extract s_64_11 s_64_9 s_64_15
        let s_64_16: Bits = (Bits::new(
            ((s_64_11) >> (s_64_9)).value(),
            u16::try_from(s_64_15).unwrap(),
        ));
        // D s_64_17: cast reint s_64_16 -> u8
        let s_64_17: bool = ((s_64_16.value()) != 0);
        // D s_64_18: call DecodePASpace(s_64_8, s_64_17)
        let s_64_18: u32 = DecodePASpace(state, tracer, s_64_8, s_64_17);
        // D s_64_19: write-var baseaddress.1 <= s_64_18
        fn_state.baseaddress._1 = s_64_18;
        // D s_64_20: read-var baseaddress.1:struct
        let s_64_20: u32 = fn_state.baseaddress._1;
        // C s_64_21: const #1u : u32
        let s_64_21: u32 = 1;
        // D s_64_22: cmp-eq s_64_20 s_64_21
        let s_64_22: bool = ((s_64_20) == (s_64_21));
        // N s_64_23: branch s_64_22 b70 b65
        if s_64_22 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#18003 <= s_65_0
        fn_state.gs_18003 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_66_0: read-var gs#18003:u8
        let s_66_0: bool = fn_state.gs_18003;
        // N s_66_1: branch s_66_0 b69 b67
        if s_66_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // N s_67_0: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // N s_68_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_69_0: const #0u : u32
        let s_69_0: u32 = 0;
        // D s_69_1: write-var baseaddress.1 <= s_69_0
        fn_state.baseaddress._1 = s_69_0;
        // N s_69_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_70_0: const #() : ()
        let s_70_0: () = ();
        // S s_70_1: call HaveSecureState(s_70_0)
        let s_70_1: bool = HaveSecureState(state, tracer, s_70_0);
        // S s_70_2: not s_70_1
        let s_70_2: bool = !s_70_1;
        // D s_70_3: write-var gs#18003 <= s_70_2
        fn_state.gs_18003 = s_70_2;
        // N s_70_4: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_71_0: read-var descriptor:bv
        let s_71_0: Bits = fn_state.descriptor;
        // D s_71_1: size-of s_71_0
        let s_71_1: u16 = s_71_0.length();
        // D s_71_2: cast zx s_71_1 -> i
        let s_71_2: i128 = (i128::try_from(s_71_1).unwrap());
        // C s_71_3: const #11s : i
        let s_71_3: i128 = 11;
        // D s_71_4: cmp-lt s_71_3 s_71_2
        let s_71_4: bool = ((s_71_3) < (s_71_2));
        // N s_71_5: assert s_71_4
        let s_71_5: () = assert!(s_71_4);
        // D s_71_6: read-var descriptor:bv
        let s_71_6: Bits = fn_state.descriptor;
        // D s_71_7: size-of s_71_6
        let s_71_7: u16 = s_71_6.length();
        // D s_71_8: cast zx s_71_7 -> i
        let s_71_8: i128 = (i128::try_from(s_71_7).unwrap());
        // C s_71_9: const #127s : i
        let s_71_9: i128 = 127;
        // D s_71_10: cmp-lt s_71_9 s_71_8
        let s_71_10: bool = ((s_71_9) < (s_71_8));
        // N s_71_11: assert s_71_10
        let s_71_11: () = assert!(s_71_10);
        // C s_71_12: const #11s : i
        let s_71_12: i128 = 11;
        // D s_71_13: read-var descriptor:bv
        let s_71_13: Bits = fn_state.descriptor;
        // C s_71_14: const #1u : u64
        let s_71_14: u64 = 1;
        // D s_71_15: bit-extract s_71_13 s_71_12 s_71_14
        let s_71_15: Bits = (Bits::new(
            ((s_71_13) >> (s_71_12)).value(),
            u16::try_from(s_71_14).unwrap(),
        ));
        // D s_71_16: cast reint s_71_15 -> u8
        let s_71_16: bool = ((s_71_15.value()) != 0);
        // C s_71_17: const #0s : i
        let s_71_17: i128 = 0;
        // C s_71_18: const #0u : u64
        let s_71_18: u64 = 0;
        // D s_71_19: cast zx s_71_16 -> u64
        let s_71_19: u64 = (s_71_16 as u64);
        // C s_71_20: const #1u : u64
        let s_71_20: u64 = 1;
        // D s_71_21: and s_71_19 s_71_20
        let s_71_21: u64 = ((s_71_19) & (s_71_20));
        // D s_71_22: cmp-eq s_71_21 s_71_20
        let s_71_22: bool = ((s_71_21) == (s_71_20));
        // D s_71_23: lsl s_71_19 s_71_17
        let s_71_23: u64 = s_71_19 << s_71_17;
        // D s_71_24: or s_71_18 s_71_23
        let s_71_24: u64 = ((s_71_18) | (s_71_23));
        // D s_71_25: cmpl s_71_23
        let s_71_25: u64 = !s_71_23;
        // D s_71_26: and s_71_18 s_71_25
        let s_71_26: u64 = ((s_71_18) & (s_71_25));
        // D s_71_27: select s_71_22 s_71_24 s_71_26
        let s_71_27: u64 = if s_71_22 { s_71_24 } else { s_71_26 };
        // D s_71_28: cast trunc s_71_27 -> u8
        let s_71_28: bool = ((s_71_27) != 0);
        // C s_71_29: const #127s : i
        let s_71_29: i128 = 127;
        // D s_71_30: read-var descriptor:bv
        let s_71_30: Bits = fn_state.descriptor;
        // C s_71_31: const #1u : u64
        let s_71_31: u64 = 1;
        // D s_71_32: bit-extract s_71_30 s_71_29 s_71_31
        let s_71_32: Bits = (Bits::new(
            ((s_71_30) >> (s_71_29)).value(),
            u16::try_from(s_71_31).unwrap(),
        ));
        // D s_71_33: cast reint s_71_32 -> u8
        let s_71_33: bool = ((s_71_32.value()) != 0);
        // C s_71_34: const #0s : i
        let s_71_34: i128 = 0;
        // C s_71_35: const #0u : u64
        let s_71_35: u64 = 0;
        // D s_71_36: cast zx s_71_33 -> u64
        let s_71_36: u64 = (s_71_33 as u64);
        // C s_71_37: const #1u : u64
        let s_71_37: u64 = 1;
        // D s_71_38: and s_71_36 s_71_37
        let s_71_38: u64 = ((s_71_36) & (s_71_37));
        // D s_71_39: cmp-eq s_71_38 s_71_37
        let s_71_39: bool = ((s_71_38) == (s_71_37));
        // D s_71_40: lsl s_71_36 s_71_34
        let s_71_40: u64 = s_71_36 << s_71_34;
        // D s_71_41: or s_71_35 s_71_40
        let s_71_41: u64 = ((s_71_35) | (s_71_40));
        // D s_71_42: cmpl s_71_40
        let s_71_42: u64 = !s_71_40;
        // D s_71_43: and s_71_35 s_71_42
        let s_71_43: u64 = ((s_71_35) & (s_71_42));
        // D s_71_44: select s_71_39 s_71_41 s_71_43
        let s_71_44: u64 = if s_71_39 { s_71_41 } else { s_71_43 };
        // D s_71_45: cast trunc s_71_44 -> u8
        let s_71_45: bool = ((s_71_44) != 0);
        // D s_71_46: cast zx s_71_28 -> bv
        let s_71_46: Bits = Bits::new(s_71_28 as u128, 1u16);
        // D s_71_47: cast zx s_71_45 -> bv
        let s_71_47: Bits = Bits::new(s_71_45 as u128, 1u16);
        // D s_71_48: cast reint s_71_46 -> u128
        let s_71_48: u128 = (s_71_46.value() as u128);
        // D s_71_49: size-of s_71_46
        let s_71_49: u16 = s_71_46.length();
        // D s_71_50: cast reint s_71_47 -> u128
        let s_71_50: u128 = (s_71_47.value() as u128);
        // D s_71_51: size-of s_71_47
        let s_71_51: u16 = s_71_47.length();
        // D s_71_52: lsl s_71_48 s_71_51
        let s_71_52: u128 = s_71_48 << s_71_51;
        // D s_71_53: or s_71_52 s_71_50
        let s_71_53: u128 = ((s_71_52) | (s_71_50));
        // D s_71_54: add s_71_49 s_71_51
        let s_71_54: u16 = (s_71_49 + s_71_51);
        // D s_71_55: create-bits s_71_53 s_71_54
        let s_71_55: Bits = Bits::new(s_71_53, s_71_54);
        // D s_71_56: cast reint s_71_55 -> u8
        let s_71_56: u8 = (s_71_55.value() as u8);
        // D s_71_57: write-var split_vec <= s_71_56
        fn_state.split_vec = s_71_56;
        // N s_71_58: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_72_0: read-var walkparams.3:struct
        let s_72_0: bool = fn_state.walkparams._3;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 1u16);
        // C s_72_2: const #1u : u8
        let s_72_2: bool = true;
        // C s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_4: cmp-eq s_72_1 s_72_3
        let s_72_4: bool = ((s_72_1) == (s_72_3));
        // N s_72_5: branch s_72_4 b78 b73
        if s_72_4 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_73_0: read-var descriptor:bv
        let s_73_0: Bits = fn_state.descriptor;
        // D s_73_1: size-of s_73_0
        let s_73_1: u16 = s_73_0.length();
        // D s_73_2: cast zx s_73_1 -> i
        let s_73_2: i128 = (i128::try_from(s_73_1).unwrap());
        // C s_73_3: const #5s : i
        let s_73_3: i128 = 5;
        // D s_73_4: cmp-lt s_73_3 s_73_2
        let s_73_4: bool = ((s_73_3) < (s_73_2));
        // N s_73_5: assert s_73_4
        let s_73_5: () = assert!(s_73_4);
        // C s_73_6: const #5s : i
        let s_73_6: i128 = 5;
        // D s_73_7: read-var descriptor:bv
        let s_73_7: Bits = fn_state.descriptor;
        // C s_73_8: const #1u : u64
        let s_73_8: u64 = 1;
        // D s_73_9: bit-extract s_73_7 s_73_6 s_73_8
        let s_73_9: Bits = (Bits::new(
            ((s_73_7) >> (s_73_6)).value(),
            u16::try_from(s_73_8).unwrap(),
        ));
        // D s_73_10: cast reint s_73_9 -> u8
        let s_73_10: bool = ((s_73_9.value()) != 0);
        // C s_73_11: const #0s : i
        let s_73_11: i128 = 0;
        // C s_73_12: const #0u : u64
        let s_73_12: u64 = 0;
        // D s_73_13: cast zx s_73_10 -> u64
        let s_73_13: u64 = (s_73_10 as u64);
        // C s_73_14: const #1u : u64
        let s_73_14: u64 = 1;
        // D s_73_15: and s_73_13 s_73_14
        let s_73_15: u64 = ((s_73_13) & (s_73_14));
        // D s_73_16: cmp-eq s_73_15 s_73_14
        let s_73_16: bool = ((s_73_15) == (s_73_14));
        // D s_73_17: lsl s_73_13 s_73_11
        let s_73_17: u64 = s_73_13 << s_73_11;
        // D s_73_18: or s_73_12 s_73_17
        let s_73_18: u64 = ((s_73_12) | (s_73_17));
        // D s_73_19: cmpl s_73_17
        let s_73_19: u64 = !s_73_17;
        // D s_73_20: and s_73_12 s_73_19
        let s_73_20: u64 = ((s_73_12) & (s_73_19));
        // D s_73_21: select s_73_16 s_73_18 s_73_20
        let s_73_21: u64 = if s_73_16 { s_73_18 } else { s_73_20 };
        // D s_73_22: cast trunc s_73_21 -> u8
        let s_73_22: bool = ((s_73_21) != 0);
        // D s_73_23: write-var ns <= s_73_22
        fn_state.ns = s_73_22;
        // N s_73_24: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_74_0: read-var ns:u8
        let s_74_0: bool = fn_state.ns;
        // D s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 1u16);
        // C s_74_2: const #0u : u8
        let s_74_2: bool = false;
        // C s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 1u16);
        // D s_74_4: cmp-eq s_74_1 s_74_3
        let s_74_4: bool = ((s_74_1) == (s_74_3));
        // N s_74_5: branch s_74_4 b77 b75
        if s_74_4 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_75_0: const #0u : u32
        let s_75_0: u32 = 0;
        // D s_75_1: write-var baseaddress.1 <= s_75_0
        fn_state.baseaddress._1 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // N s_76_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_77_0: const #1u : u32
        let s_77_0: u32 = 1;
        // D s_77_1: write-var baseaddress.1 <= s_77_0
        fn_state.baseaddress._1 = s_77_0;
        // N s_77_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_78_0: read-var descriptor:bv
        let s_78_0: Bits = fn_state.descriptor;
        // D s_78_1: size-of s_78_0
        let s_78_1: u16 = s_78_0.length();
        // D s_78_2: cast zx s_78_1 -> i
        let s_78_2: i128 = (i128::try_from(s_78_1).unwrap());
        // C s_78_3: const #127s : i
        let s_78_3: i128 = 127;
        // D s_78_4: cmp-lt s_78_3 s_78_2
        let s_78_4: bool = ((s_78_3) < (s_78_2));
        // N s_78_5: assert s_78_4
        let s_78_5: () = assert!(s_78_4);
        // C s_78_6: const #127s : i
        let s_78_6: i128 = 127;
        // D s_78_7: read-var descriptor:bv
        let s_78_7: Bits = fn_state.descriptor;
        // C s_78_8: const #1u : u64
        let s_78_8: u64 = 1;
        // D s_78_9: bit-extract s_78_7 s_78_6 s_78_8
        let s_78_9: Bits = (Bits::new(
            ((s_78_7) >> (s_78_6)).value(),
            u16::try_from(s_78_8).unwrap(),
        ));
        // D s_78_10: cast reint s_78_9 -> u8
        let s_78_10: bool = ((s_78_9.value()) != 0);
        // C s_78_11: const #0s : i
        let s_78_11: i128 = 0;
        // C s_78_12: const #0u : u64
        let s_78_12: u64 = 0;
        // D s_78_13: cast zx s_78_10 -> u64
        let s_78_13: u64 = (s_78_10 as u64);
        // C s_78_14: const #1u : u64
        let s_78_14: u64 = 1;
        // D s_78_15: and s_78_13 s_78_14
        let s_78_15: u64 = ((s_78_13) & (s_78_14));
        // D s_78_16: cmp-eq s_78_15 s_78_14
        let s_78_16: bool = ((s_78_15) == (s_78_14));
        // D s_78_17: lsl s_78_13 s_78_11
        let s_78_17: u64 = s_78_13 << s_78_11;
        // D s_78_18: or s_78_12 s_78_17
        let s_78_18: u64 = ((s_78_12) | (s_78_17));
        // D s_78_19: cmpl s_78_17
        let s_78_19: u64 = !s_78_17;
        // D s_78_20: and s_78_12 s_78_19
        let s_78_20: u64 = ((s_78_12) & (s_78_19));
        // D s_78_21: select s_78_16 s_78_18 s_78_20
        let s_78_21: u64 = if s_78_16 { s_78_18 } else { s_78_20 };
        // D s_78_22: cast trunc s_78_21 -> u8
        let s_78_22: bool = ((s_78_21) != 0);
        // D s_78_23: write-var ns <= s_78_22
        fn_state.ns = s_78_22;
        // N s_78_24: jump b74
        return block_74(state, tracer, fn_state);
    }
}
